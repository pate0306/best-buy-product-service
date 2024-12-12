use crate::model::Product;
use crate::configuration::Settings;
 
pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Dell XPS 13 Laptop".to_string(),
            price: 1299.99,
            description: "Experience exceptional performance with the Dell XPS 13. Powered by Intel Core processors and boasting an InfinityEdge display.".to_string(),
            image: "/dell_xps13.png".to_string()
        },
        Product {
            id: 2,
            name: "Samsung Galaxy S23 Ultra Smartphone".to_string(),
            price: 1199.99,
            description: "Stay connected with the Samsung Galaxy S23 Ultra. Featuring a powerful processor, a stunning AMOLED display, and advanced camera features.".to_string(),
            image: "/galaxy_s23_ultra.png".to_string()
        },
        Product {
            id: 3,
            name: "Roku Ultra 4K Streaming Device".to_string(),
            price: 99.99,
            description: "Stream your favorite shows with the Roku Ultra. Enjoy 4K HDR streaming, Dolby Vision, and voice search capabilities.".to_string(),
            image: "/roku_ultra.png".to_string()
        },
        Product {
            id: 4,
            name: "Canon EOS R50 Mirrorless Camera".to_string(),
            price: 999.99,
            description: "Capture high-quality photos and videos with the Canon EOS R50. Lightweight and versatile, perfect for creators on the go.".to_string(),
            image: "/canon_eos_r50.png".to_string()
        },
        Product {
            id: 5,
            name: "Breville Barista Express Espresso Machine".to_string(),
            price: 699.99,
            description: "Make café-quality coffee at home with the Breville Barista Express. Features a built-in grinder and precise espresso extraction.".to_string(),
            image: "/breville_espresso.png".to_string()
        },
        Product {
            id: 6,
            name: "iRobot Roomba j7+ Robot Vacuum".to_string(),
            price: 799.99,
            description: "Keep your floors clean effortlessly with the iRobot Roomba j7+. Smart navigation and a self-emptying bin for ultimate convenience.".to_string(),
            image: "/roomba_j7.png".to_string()
        },
        Product {
            id: 7,
            name: "Microsoft Xbox Series X Console".to_string(),
            price: 499.99,
            description: "Dive into immersive gaming with the Xbox Series X. Features 4K resolution, high frame rates, and a vast library of games.".to_string(),
            image: "/xbox_series_x.png".to_string()
        },
        Product {
            id: 8,
            name: "JBL Flip 6 Portable Bluetooth Speaker".to_string(),
            price: 129.99,
            description: "Take your music anywhere with the JBL Flip 6. Offers powerful sound, water resistance, and up to 12 hours of playtime.".to_string(),
            image: "/jbl_flip6.png".to_string()
        },
        Product {
            id: 9,
            name: "Nest Learning Thermostat (3rd Gen)".to_string(),
            price: 249.99,
            description: "Save energy and stay comfortable with the Nest Learning Thermostat. Adapts to your schedule and can be controlled remotely.".to_string(),
            image: "/nest_thermostat.png".to_string()
        },
        Product {
            id: 10,
            name: "Anker PowerCore 26800 Portable Charger".to_string(),
            price: 59.99,
            description: "Keep your devices charged on the go with the Anker PowerCore 26800. High capacity and fast charging for all your gadgets.".to_string(),
            image: "/anker_powercore.png".to_string()
        }
    ]
}