use yew::prelude::*;

pub struct Home {
    state: State,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let products = vec![
            Product {
                id: 1,
                name: "Apple".to_string(),
                description: "An apple a day keeps the doctor away".to_string(),
                image: "https://p1.hiclipart.com/preview/616/544/999/fruit-and-vegetable-red-apple-fruit-png-clipart.jpg".to_string(),
                price: 3.65,
            },
            Product {
                id: 2,
                name: "Banana".to_string(),
                description: "An old banana leaf was once young and green".to_string(),
                image: "https://cdn.iconscout.com/icon/free/png-256/bananas-55-1176320.png".to_string(),
                price: 7.99,
            },
        ];
        
        Self {
            state: State {
                products,
            }
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let products: Vec<Html> = self
            .state
            .products
            .iter()
            .map(|product: &Product| {
                html! {
                    <div>
                        <img src={&product.image}/>
                        <div>{&product.name}</div>
                        <div>{"$"}{&product.price}</div>
                    </div>
                }
            })
            .collect();
        
            html! { <span>{products}</span> }
    }
}

struct Product {
    id: i32,
    name: String,
    description: String,
    image: String,
    price: f64,
}

struct State {
    products: Vec<Product>,
}