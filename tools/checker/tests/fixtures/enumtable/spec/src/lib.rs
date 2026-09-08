use patterns::because;

pub enum Rate {
    Old = 15,
    New = 20,
}
because!(Rate, "the two daily rates the desk has used, oldest first, as a vocabulary");
