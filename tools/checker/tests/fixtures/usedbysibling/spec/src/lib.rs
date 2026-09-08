use patterns::because;

pub struct Loan;
because!(Loan, "one book in the hands of one member until it comes back");
pub struct State {
    pub loans: Loan,
}
because!(State, "everything the branch knows about what is out on loan");
