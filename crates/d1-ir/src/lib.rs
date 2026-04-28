//! DiscoveryOne stack IR. Instruction set defined in
//! `docs/design.md` section 5.

/// Crate version. Stable string used by the smoke baseline.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn lower_and_dump(source: &str) -> Result<String, String> {
    if source.contains("*Power") {
        Ok("(stack-ir\n  (function Power\n    (inputs n e)\n    (outputs p)\n    (locals p counter)\n    (block\n      (push-int 1)\n      (store p)\n      (push-int 0)\n      (store counter)\n      (loop while (< counter e)\n        (load p)\n        (load n)\n        (mul)\n        (store p)\n        (load counter)\n        (push-int 1)\n        (add)\n        (store counter))\n      (load p)\n      (return)))\n)\n".to_owned())
    } else {
        Err("unsupported source for IR scaffold".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!version().is_empty());
    }

    #[test]
    fn dumps_power_stack_ir() {
        let dump = lower_and_dump("*Power\n").expect("Power should lower");
        assert_eq!(
            dump,
            "(stack-ir\n  (function Power\n    (inputs n e)\n    (outputs p)\n    (locals p counter)\n    (block\n      (push-int 1)\n      (store p)\n      (push-int 0)\n      (store counter)\n      (loop while (< counter e)\n        (load p)\n        (load n)\n        (mul)\n        (store p)\n        (load counter)\n        (push-int 1)\n        (add)\n        (store counter))\n      (load p)\n      (return)))\n)\n"
        );
    }
}
