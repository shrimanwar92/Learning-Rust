#[derive(Debug)]
#[derive(Copy, Clone)]
struct Product {
	name: &'static str,
	unit_price: f64,
}

#[derive(Debug)]
struct Item {
	item: Product,
	quantity: i32,
	total_price: f64,
}

impl Product {
	fn seed_data() -> Vec<Product> {
		let mut products = Vec::new();
		products.push(Product {name: "Lux Soap", unit_price: 20.0});
		products.push(Product {name: "Honey", unit_price: 120.0});
		products.push(Product {name: "Hand wash", unit_price: 40.0});
		products.push(Product {name: "Brush", unit_price: 100.0});
		products
	}
}

impl Item {
	fn new(product: Product, no_of_items: i32) -> Item {
		Item {
			item: product,
			quantity: no_of_items,
			total_price: no_of_items as f64 * &product.unit_price,
		}
	}

	fn add_to_cart(items: Vec<Item>) -> f64 {
		let mut sum: f64 = 0.0;
		for item in &items {
			sum += item.total_price;
		}
		sum
	}

	fn update_item(&mut self, new_quantity: i32) -> Item {
		self.quantity = new_quantity;
		self.total_price = self.quantity as f64 * self.item.unit_price; 
		Item {
			item: self.item,
			quantity: self.quantity,
			total_price: self.total_price
		}
	}
}

fn main() {
    let products = Product::seed_data();

    let mut i1 = Item::new(products[0], 3);
    let mut i2 = Item::new(products[1], 6);

    let i1 = i1.update_item(4);
    println!("{:?}", i1);

    let i2 = i2.update_item(10);
    println!("{:?}", i2);

    let total = Item::add_to_cart(vec![i1, i2]);
    println!("{:?}", total);
}
