#[cfg(test)]
mod stress_tests {
    use owlsol_cli::core::fee_optimizer::*;
    use futures::future::join_all;
    use tokio;

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore]
    async fn test_fee_optimizer_stress_many_calls() {
        let fees = vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000];
        let strat = FeeStrategy::Percentile(80);
        let tasks: Vec<_> = (0..100)
            .map(|_| {
                let fees = fees.clone();
                let strat = strat.clone();
                tokio::spawn(async move {
                    get_optimal_priority_fee(&fees, &strat)
                })
            })
            .collect();
        let results = join_all(tasks).await;
        for res in results {
            assert!(res.is_ok());
            let fee = res.unwrap();
            assert!(fee > 0);
        }
    }
}
