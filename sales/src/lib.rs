#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().find(|(name, _)| *name == ele) {
            self.items.push((product.0.clone(), product.1));
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut sorted_items: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        sorted_items.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let original_total: f32 = sorted_items.iter().sum();
        let free_count = sorted_items.len() / 3;
        let discount_total: f32 = sorted_items.iter().take(free_count).sum();
        let total_after_promotion = original_total - discount_total;

        let adjustment_factor = total_after_promotion / original_total;
        let mut adjusted_prices: Vec<f32> = sorted_items
            .iter()
            .map(|price| (price * adjustment_factor * 100.0).round() / 100.0)
            .collect();

        adjusted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_item() {
        let store = Store::new(vec![
            (String::from("product A"), 1.23),
            (String::from("product B"), 23.1),
            (String::from("product C"), 3.12),
        ]);

        let mut cart = Cart::new();
        cart.insert_item(&store, String::from("product A"));
        cart.insert_item(&store, String::from("product B"));

        assert_eq!(cart.items.len(), 2);
        assert_eq!(cart.items[0].0, "product A");
        assert_eq!(cart.items[1].0, "product B");
    }
    #[test]
    fn test_generate_receipt() {
        let store = Store::new(vec![
            (String::from("product A"), 1.23),
            (String::from("product B"), 23.1),
            (String::from("product C"), 3.12),
        ]);

        let mut cart = Cart::new();
        cart.insert_item(&store, String::from("product A"));
        cart.insert_item(&store, String::from("product B"));
        cart.insert_item(&store, String::from("product C"));

        let receipt = cart.generate_receipt();
        assert_eq!(receipt.len(), 3);
    }
}