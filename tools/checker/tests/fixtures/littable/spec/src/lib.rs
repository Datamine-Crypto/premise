use patterns::because;

pub const TABLE: [u32; 2] = [15, 20];
because!(TABLE, "the two daily rates the desk has used, oldest first");
