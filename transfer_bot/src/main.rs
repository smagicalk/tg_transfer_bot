fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(transfer_bot::TOKIO_WORKER_STACK_SIZE)
        .build()?
        .block_on(transfer_bot::run())
}
