use crate::workers::Task;
mod collection;
mod game;
mod nft;
mod trade;

pub fn create_tasks() -> Vec<Task> {
	let mut tasks = vec![];

	tasks.push(nft::on_mint_nft_task());
	tasks.push(game::on_game_created_task());

	tasks
}
