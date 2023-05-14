(define-data-var last-transaction-id u64 (default 0))

(define-read-only (get-last-transaction-id)
  last-transaction-id)

(define-public (send-funds (from principal) (to principal) (amount uint64))
  (let ((balance (get-balance from)))
    (if (>= balance amount)
      (begin
        (transfer amount from to)
        (set-data-var last-transaction-id (add u64 (get-data-var last-transaction-id) 1))
        (get-data-var last-transaction-id))
      (err "Not enough balance"))))
  
(define-read-only (get-balance (address principal))
  (balance (contract-caller)))
