sdl with grid layout and window management to help build apps or video games.

you can see an example of usage at https://github.com/RedYou1/PVZ

# Getting Started
use the function to create the window and run the application
``` Rust
red_sdl::run(
    window title,
    fps,
    width,
    height,
    window builder: Fn(&mut WindowBuilder) -> &mut WindowBuilder, //ex:|window| window.fullscreen_desktop().resizable(),
    your window constructor: Fn(&mut Canvas<Window>) -> Result<impl GameWindow, String>,
)
```
## Order of called function
init when created
loop the rest for each frame in this order:
1. init_frame
2. event
3. update
4. draw

# Elements
## Grid
Let you place elements in a grid without overlaps.
### Parent new
You can use his empty function for your new function of your parent element / window as long as you create it in the init function.
### Grid new
Usually used for dynamic number of columns or rows
``` Rust
Grid::new(
    ptr to parent,//self
    Vec of your columns,
    Vec of you rows,
    HashMap of Pos => Box of dyn GridChildren<ParentType>
)
```

### Macro to create a Grid element
Usually used for static number of columns or rows
``` Rust
simple_grid!(
    RefToParent,//self
    ParentType,//Win
    //Your columns
    ColType::Px(min 1.),
    ColType::Ratio(minex 0.),
    ...; // char end of columns
    //Your rows
    RowType::Px(min 1.),
    RowType::Ratio(minex 0.),
    ...; // char end of rows
    Pos{x:min 0,y:min 0} => sub element,
    ...
)
```

## RefElement
Let you have the ownership of a sub element inside of another element.
exemple:
- Window contains SubElement
- Grid contains RefElement
- RefElement contains a reference to SubElement
- 
- Grid can use SubElement as if it was him
- Window can have an easy access to SubElement
### Remark
Can be prone to errors if the sub element don't have the same lifetime of the parent.
### new
``` Rust
RefElement::new(ref of the sub element)
```

## ScrollView
Let you have an unrestrained sized sub element inside your restrained sized Window/SubElement.
### new
``` Rust
ScrollView::new(
    the child element,
    the child width minex 0.,
    the child height minex 0.,
    Func for the scrollbar color: Box<dyn Fn(&Parent, &Element) -> Color>
)
```

## UIRect
Let you design a rectangle with the builder pattern.
### new
``` Rust
UIRect::new(
    state function: Box<dyn Fn(&Parent, &Element) -> StateEnum>,
    back color function: Box<dyn Fn(&Parent, &Element) -> Color>
)
```
### action
Setup an action function. Get called in the event function.
It gets called when the mouse is hovering over the element and the left mouse button is down.
``` Rust
Box<dyn Fn(
    &mut Parent,
    &UIRect,
    mouse_x,
    mouse_y,
    &mut Canvas) -> Result<(), String>
```
### text
Setup the text to write in the rect. Get called in the draw function.
``` Rust
Box<dyn Fn(&Parent, &Element) 
-> Result<(Option<UIString>, Color), String>>
```
### image
Draw an image behind the text. Get called in the draw function.
``` Rust
Box<dyn Fn(&Parent, &Element)
-> Result<&'static Texture<'static>, String>>
```

## TextBox
Let the user enter text inside this element.
### new
``` Rust
TextBox::new(
    The id of the element,
    ref to the selected element field,
    the font,
    ref to the current text in the element,
    state function: Box<dyn Fn(&Parent, &Element) -> StateEnum>,
    select_color function: Box<dyn Fn(&Parent, &Element) -> Color>,
    line_color function: Box<dyn Fn(&Parent, &Element) -> Color>,
    front_color function: Box<dyn Fn(&Parent, &Element) -> Color>,
    back_color function: Box<dyn Fn(&Parent, &Element) -> Color>,
)
```