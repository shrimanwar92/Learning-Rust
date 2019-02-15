#[derive(Debug)]
#[derive(Copy, Clone)]
struct Product {
	name: &'static str,
	unit_price: f64,
}

#[derive(Debug, Copy, Clone)]
struct Item {
	item: Product,
	quantity: i32,
	total_price: f64,
}

#[derive(Debug)]
struct Cart {
	items: Vec<Item>
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

	fn update_item(&mut self, new_quantity: i32) -> Self {
		self.quantity = new_quantity;
		self.total_price = self.quantity as f64 * self.item.unit_price;
		*self
	}
}

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        self == other
    }
}

impl Cart {
	fn new() -> Cart{
		Cart {
			items: Vec::new()
		}
	}

	fn add(&mut self, item: Item) {
		self.items.push(item);
	}

	fn update(&mut self, item1: Item, quantity: i32) {
		/*let u_item = item.update_item(quantity);
		println!("{:?}", u_item);*/

		for mut item in &mut self.items {
			if item.item.name == item1.item.name {
				item.quantity = quantity;
				item.total_price = item.quantity as f64 * item.item.unit_price;
			}
		}
	}

	fn calculate_total(&self) -> f64 {
		let mut sum: f64 = 0.0;
		for item in &self.items {
			sum += item.total_price;
		}
		sum
	}
}

fn main() {
    let products = Product::seed_data();

    let mut i1 = Item::new(products[0], 3);
    let mut i2 = Item::new(products[1], 6);
    
    // individual item update
    let i1 = i1.update_item(4);
    println!("{:?}", i1);

    let i2 = i2.update_item(10);
    println!("{:?}", i2);

    let mut cart = Cart::new();
    cart.add(i1);
    cart.add(i2);

    // update item in cart array
    cart.update(i1, 10);
    cart.update(i1, 12);
    println!("{:?}", cart);

    println!("{:?}", cart.calculate_total());
}
