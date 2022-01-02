use dismem::job::Job;
use dismem::job_factory::JobCollection;
use dismem::registry::NodeRegistry;
use dismem::scheduler::Scheduler;

#[cfg(test)]
mod test_scheduler {
    use super::*;
    
    fn registry_init_homogeneous(
        nodes: usize,
        cores: f32,
        memory: f32,
    ) -> Result<NodeRegistry, String> {
        let mut reg = NodeRegistry::new();
    
        for i in 0..nodes {
            reg.new_node(&format!("{}", i), cores, memory)?;
        }
    
        Ok(reg)
    }
    
    fn jobfactory_init_homogeneous(
        jobs_created: &Vec<f32>,
        cores: f32,
        memory: f32,
        duration: f32,
        can_borrow: bool,
    ) -> JobCollection {
        let jobs: Vec<_> = jobs_created
            .iter()
            .map(|created| Job::new(cores, memory, duration, can_borrow, *created))
            .collect();
    
        JobCollection::new(jobs)
    }
    
    
    #[test]
    fn scheduler_vanilla() -> Result<(), String> {
        let reg = registry_init_homogeneous(2, 1., 1.)?;
        let job_created: Vec<_> = vec![0.0, 1., 2., 3.];
    
        let job_factory = jobfactory_init_homogeneous(&job_created, 1.0, 1.0, 5.0, false);
    
        let mut sched = Scheduler::new(reg, job_factory);
    
        while sched.tick() {}
    
        assert!(sched.job_factory.jobs_done.len() == 4);
    
        Ok(())
    }
    
    #[test]
    fn schedule_vanilla_large() -> Result<(), String> {
        let reg = registry_init_homogeneous(100, 1., 1.)?;
        let num_jobs = 100;
    
        let job_created: Vec<_> = vec![0.0; num_jobs];
    
        let job_factory = jobfactory_init_homogeneous(&job_created, 1.0, 1.0, 5.0, false);
    
        let mut sched = Scheduler::new(reg, job_factory);
    
        while sched.tick() {}
    
        assert_eq!(sched.job_factory.jobs_done.len(), num_jobs);
    
        assert_eq!(sched.now, 5.0);
    
        Ok(())
    }
    
}

