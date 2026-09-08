use patterns::because;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Region {
    Domestic,
    International,
    Transit,
}
because!(Region, "billing tracks goods in transit because VAT accrues before delivery, which fulfilment never sees");
