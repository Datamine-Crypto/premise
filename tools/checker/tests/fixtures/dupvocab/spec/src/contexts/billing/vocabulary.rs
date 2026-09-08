use patterns::because;

pub enum Phase {
    Closed,
    Open,
}
because!(Phase, "billing open and closed tracks invoice state, not order state");
