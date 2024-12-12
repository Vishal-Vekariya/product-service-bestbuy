use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 55\" 4K UHD Smart TV".to_string(),
            price: 549.99,
            description: "Experience crystal-clear picture quality with the Samsung 55-inch 4K UHD Smart TV. Packed with HDR10+ and voice assistant compatibility.".to_string(),
            image: "/tv.png".to_string()
        },
        Product {
            id: 2,
            name: "Apple MacBook Air 13\" M2 Chip".to_string(),
            price: 1199.99,
            description: "The Apple MacBook Air with M2 chip redefines performance in a sleek and lightweight design. Perfect for students and professionals.".to_string(),
            image: "/macbook.png".to_string()
        },
        Product {
            id: 3,
            name: "Sony WH-1000XM5 Noise Cancelling Headphones".to_string(),
            price: 399.99,
            description: "Immerse yourself in pure sound with Sony's WH-1000XM5 over-ear headphones, featuring leading noise-cancellation and 30-hour battery life.".to_string(),
            image: "/headphones.png".to_string()
        },
        Product {
            id: 4,
            name: "Microsoft Xbox Series X".to_string(),
            price: 499.99,
            description: "Power your gaming experience with the Xbox Series X. Enjoy stunning graphics, fast load times, and an extensive library of games.".to_string(),
            image: "/xbox.png".to_string()
        }
    ]
}