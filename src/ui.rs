use std::rc::Rc;
use buoy::Context;
use buoy::render::color;
use buoy::element::{IntoUIElement, IntoObj, Stub, StubImpl, Widget, Id, Filter, UIElement};
use buoy::elements::{
    fill::SolidFill,
    min_max::{MinMax, VAlign},
    border::BlockBorder,
    list::List,
    hover::Hover,
};

#[derive(Clone, Copy)]
pub struct BlueBox;
impl StubImpl for BlueBox {
    fn generate(self, ctx: &mut Context) {
        let id = ctx.element_id();

        BlockBorder::uniform(10_f32)
        .color(color::RGBA8(0x10_C0_C9_FF))
        .into_obj(id.append_str("border"))
        .push(ctx);

            Hover::new(ctx, Rc::new(move |_| println!("Hovered on element {}!", id)))
            .into_obj(id.append_str("hover"))
            .push(ctx);

                SolidFill::new(color::constants::WHITE)
                .into_obj(id.append_str("fill"))
                .push(ctx);

                    MinMax::default().width(20_f32).height(10_f32)
                    .into_obj(id.append_str("inner"))
                    .push(ctx).pop();

                ctx.pop(); // fill
            ctx.pop(); // hover
        ctx.pop(); // border
    }
}

impl IntoUIElement for BlueBox {
    type Target = Stub<BlueBox>;
}

#[derive(Clone, Copy)]
pub struct TestStub;
impl StubImpl for TestStub {
    fn generate(self, ctx: &mut Context) {
        List::bottom_to_top().into_obj(Id::str("TestGenerator_stack")).push(ctx);

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).into_obj(Id::str("BlueBox_1_padding")).push(ctx);
                MinMax::default().height(100_f32).v_align(VAlign::Center).into_obj(Id::str("BlueBox_1_max")).push(ctx);
                    BlueBox.into_obj(Id::str("BlueBox_1")).push(ctx).pop();
                ctx.pop(); // BlueBox_1_max
            ctx.pop(); // BlueBox_1_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).into_obj(Id::str("BlueBox_2_padding")).push(ctx);
                MinMax::default().height(200_f32).v_align(VAlign::Bottom).into_obj(Id::str("BlueBox_2_max")).push(ctx);
                    BlueBox.into_obj(Id::str("BlueBox_2")).push(ctx).pop();
                ctx.pop(); // BlueBox_2_max
            ctx.pop(); // BlueBox_2_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).into_obj(Id::str("BlueBox_3_padding")).push(ctx);
                MinMax::default().height(300_f32).v_align(VAlign::Center).into_obj(Id::str("BlueBox_3_max")).push(ctx);
                    BlueBox.into_obj(Id::str("BlueBox_3")).push(ctx).pop();
                ctx.pop(); // BlueBox_3_max
            ctx.pop(); // BlueBox_3_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).into_obj(Id::str("BlueBox_4_padding")).push(ctx);
                MinMax::default().height(400_f32).v_align(VAlign::Top).into_obj(Id::str("BlueBox_4_max")).push(ctx);
                    BlueBox.into_obj(Id::str("BlueBox_4")).push(ctx).pop();
                ctx.pop(); // BlueBox_4_max
            ctx.pop(); // BlueBox_4_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).into_obj(Id::str("BlueBox_5_padding")).push(ctx);
                MinMax::default().height(500_f32).v_align(VAlign::Center).into_obj(Id::str("BlueBox_5_max")).push(ctx);
                    BlueBox.into_obj(Id::str("BlueBox_5")).push(ctx).pop();
                ctx.pop(); // BlueBox_5_max
            ctx.pop(); // BlueBox_5_padding

            // for _ in 0..100 {
            //     BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).into_obj(Id::str("BlueBox_padding")).push(ctx);
            //         MinMax::default().height(500_f32).v_align(VAlign::Center).into_obj(Id::str("BlueBox_x_max")).push(ctx);
            //             BlueBox.into_obj(Id::str("BlueBox_x")).push(ctx).pop();
            //         ctx.pop(); // BlueBox_5_max
            //     ctx.pop(); // BlueBox_5_padding
            // }

        ctx.pop(); // TestGenerator_stack
    }
}

impl IntoUIElement for TestStub {
    type Target = Stub<TestStub>;
}

#[derive(Clone, Copy)]
pub struct Fader {
    target: Id,
    value: f32,
    delta: f32,
}

impl Fader {
    pub fn new(target: Id) -> Self {
        Fader {
            target,
            value: 1_f32,
            delta: -0.01_f32,
        }
    }

    fn fade_color(&self, col: color::RGBA8) -> color::RGBA8 {
        let red = (f32::from(col.red()) * self.value) as u8;
        let green = (f32::from(col.green()) * self.value) as u8;
        let blue = (f32::from(col.blue()) * self.value) as u8;
        color::RGBA8::new(red, green, blue, 0xFF)
    }

    fn next(mut self) -> Self {
        self.value += self.delta;
        if self.value > 1_f32 {
            self.value = 1_f32;
            self.delta = -0.01_f32;
        } else if self.value < 0_f32 {
            self.value = 0_f32;
            self.delta = 0.01_f32;
        }

        self
    }
}

impl Filter for Fader {
    fn filter(&self, ctx: &mut Context, mut elem: UIElement) {
        if elem.id == self.target {
            // Modify the color
            let mut elem = elem.downcast::<Widget<BlockBorder>>().ok().unwrap();
            elem.imp.color = self.fade_color(elem.imp.color);

            // Put it back into the context
           ctx.push(elem.upcast());
                ctx.children();
            ctx.pop();

            // Create a new filter, with a different value
            ctx.next_frame(Rc::new(self.next()));
        } else {
            elem.attach_filter_post(Rc::new(*self));
            ctx.push(elem);
                ctx.children();
            ctx.pop();
        }
    }
}

#[derive(Clone, Copy)]
pub struct Grower {
    pub target: Id,
    value: f32,
    max: f32,
    min: f32,
    delta: f32,
}

impl Grower {
    pub fn new(target: Id) -> Self {
        Grower {
            target,
            value: 100_f32,
            max: 200_f32,
            min: 20_f32,
            delta: 0.5_f32,
        }
    }

    pub fn grow(&self, bounds: &mut MinMax) {
        *bounds = bounds.width(self.value);
    }

    pub fn next(mut self) -> Self {
        self.value += self.delta;
        if self.value > self.max {
            self.value = self.max;
            self.delta = -self.delta;
        } else if self.value < self.min {
            self.value = self.min;
            self.delta = -self.delta;
        }

        self
    }
}

impl Filter for Grower {
    fn filter(&self, ctx: &mut Context, mut elem: UIElement) {
        if elem.id == self.target {
            let mut elem = elem.downcast::<Widget<MinMax>>().ok().unwrap();
            self.grow(&mut *elem.imp);

            // Put it back in the context
            ctx.push(elem.upcast());
                ctx.children();
            ctx.pop();

            // Create a new filter for the next frame
            ctx.next_frame(Rc::new(self.next()));
        } else {
            elem.attach_filter_post(Rc::new(*self));
            ctx.push(elem);
                ctx.children();
            ctx.pop();
        }
    }
}
