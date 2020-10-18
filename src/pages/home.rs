use yew::prelude::*;

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
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

        let cart_products = vec![];
        
        Self {
            state: State {
                products,
                cart_products,
            },
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::AddToCart(product_id) => {
                let product = self
                    .state
                    .products
                    .iter()
                    .find(|p: &&Product| p.id == product_id)
                    .unwrap();
                let cart_product = self
                    .state
                    .cart_products
                    .iter_mut()
                    .find(|cp: &&mut CartProduct| cp.product.id == product_id);

                if let Some(cp) = cart_product {
                    cp.quantity += 1;
                } else {
                    self.state.cart_products.push(CartProduct {
                        product: product.clone(),
                        quantity: 1,
                    })
                }
                true
            }
        }
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
                let product_id = product.id;
                html! {
                    <div>
                        <img src={&product.image}/>
                        <div>{&product.name}</div>
                        <div>{"$"}{&product.price}</div>
                        <button onclick=self.link.callback(move |_| Msg::AddToCart(product_id))>{"Add To Cart"}</button>
                    </div>
                }
            })
            .collect();
        
        let cart_value = self
            .state
            .cart_products
            .iter()
            .fold(0.0, |acc, cp| acc + (cp.quantity as f64 * cp.product.price));
        
        html! {
            <div>
                <span>{format!("Cart Value: {:.2}", cart_value)}</span>
                <span>{products}</span>
            </div>
        }
    }
}

struct State {
    products: Vec<Product>,
    cart_products: Vec<CartProduct>,
}

pub enum Msg {
    AddToCart(i32),
}