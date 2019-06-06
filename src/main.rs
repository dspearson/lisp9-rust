/*
 * LISP9 Interpreter
 * Copyright 2019 Dominic Pearson <dsp@technoanimal.net>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

const VERSION: &str = "20190603";

/*
 * Tunable parameters
 */

const IMAGEFILE: &str = "ls9.image";
const IMAGESRC:  &str = "ls9.ls9";

const NNODES:     usize = 262144;
const NVCELLS:    i64 = 262144;
const NPORTS:     i32 = 20;
const TOKLEN:     i32 = 80;
const CHUNKSIZE:  i32 = 1024;
const MXMAX:      i32 = 2000;
const NTRACE:     i32 = 10;
const PRDEPTH:    i32 = 1024;

/*
 * Basic data types
 */

type Cell = i32;
type Byte = char;
type Uint = u32;

/*
 * Special Objects
 */

fn specialp(x: i32) -> bool {
    x < 0
}

const NIL:     i32 = -1;
const TRUE:    i32 = -2;
const EOFMARK: i32 = -3;
const UNDEF:   i32 = -4;
const RPAREN:  i32 = -5;
const DOT:     i32 = -6;

/*
 * Memory pools
 */

type Car = Vec<Cell>;
type Cdr = Vec<Cell>;
type Tag = Vec<Cell>;

struct NodePool {
    cars: Car,
    cdrs: Cdr,
    tags: Tag,
}

type Vectors = Vec<Cell>;
type Freelist = Vec<i32>;
type Freevec = i32;

const ATOM_TAG:   i32 = 0x01;  /* Atom, CAR = type, CDR = next */
const MARK_TAG:   i32 = 0x02;  /* Mark */
const TRAV_TAG:   i32 = 0x04;  /* Traversal */
const VECTOR_TAG: i32 = 0x08;  /* Vector, CAR = type, CDR = content */
const PORT_TAG:   i32 = 0x10;  /* Atom is an I/O port (with ATOM_TAG) */
const USED_TAG:   i32 = 0x20;  /* Port: used flag */
const LOCK_TAG:   i32 = 0x40;  /* Port: locked (do not close) */
const CONST_TAG:  i32 = 0x80;  /* Node is immutable */

macro_rules! tag {
    ($pool:expr, $n:expr) => {{
        let tags = $pool.tags;
        let index = $n as usize;
        tags[index]
    }}
}

macro_rules! car {
    ($pool:expr, $n:expr) => {{
        let cars = $pool.cars;
        let index = $n as usize;
        cars[index]
    }}
}

macro_rules! cdr {
    ($pool:expr, $n:expr) => {{
        let cdrs = $pool.cdrs;
        let index = $n as usize;
        cdrs[index]
    }}
}

macro_rules! caar {
    ($pool:expr, $n:expr) => {{
        car!(car!($pool, $n))
    }}
}

macro_rules! cadr {
    ($pool:expr, $n:expr) => {{
        car!(cdr!($pool, $n))
    }}
}

macro_rules! cdar {
    ($pool:expr, $n:expr) => {{
        cdr!(car!($pool, $n))
    }}
}

macro_rules! cddr {
    ($pool:expr, $n:expr) => {{
        cdr!(cdr!($pool, $n))
    }}
}

macro_rules! caaar {
    ($pool:expr, $n:expr) => {{
        car!(car!(car!($pool, $n)))
    }}
}

macro_rules! caadr {
    ($pool:expr, $n:expr) => {{
        car!(car!(cdr!($pool, $n)))
    }}
}

macro_rules! cadar {
    ($pool:expr, $n:expr) => {{
        car!(cdr!(car!($pool, $n)))
    }}
}

macro_rules! caddr {
    ($pool:expr, $n:expr) => {{
        car!(car!(cdr!($pool, $n)))
    }}
}

macro_rules! cdaar {
    ($pool:expr, $n:expr) => {{
        cdr!(car!(car!($pool, $n)))
    }}
}

macro_rules! cdadr {
    ($pool:expr, $n:expr) => {{
        cdr!(car!(cdr!($pool, $n)))
    }}
}

macro_rules! cddar {
    ($pool:expr, $n:expr) => {{
        cdr!(cdr!(car!($pool, $n)))
    }}
}

macro_rules! cdddr {
    ($pool:expr, $n:expr) => {{
        car!(cdr!(cdr!($pool, $n)))
    }}
}

macro_rules! caaaar {
    ($pool:expr, $n:expr) => {{
        car!(car!(car!(car!($pool, $n))))
    }}
}

macro_rules! caaadr {
    ($pool:expr, $n:expr) => {{
        car!(car!(car!(cdr!($pool, $n))))
    }}
}

macro_rules! caadar {
    ($pool:expr, $n:expr) => {{
        car!(car!(cdr!(car!($pool, $n))))
    }}
}

macro_rules! caaddr {
    ($pool:expr, $n:expr) => {{
        car!(car!(cdr!(cdr!($pool, $n))))
    }}
}

macro_rules! cadaar {
    ($pool:expr, $n:expr) => {{
        car!(cdr!(car!(car!($pool, $n))))
    }}
}

macro_rules! cadadr {
    ($pool:expr, $n:expr) => {{
        car!(cdr!(car!(cdr!($pool, $n))))
    }}
}

macro_rules! caddar {
    ($pool:expr, $n:expr) => {{
        car!(cdr!(cdr!(car!($pool, $n))))
    }}
}

macro_rules! cadddr {
    ($pool:expr, $n:expr) => {{
        car!(cdr!(cdr!(cdr!($pool, $n))))
    }}
}

macro_rules! cdaaar {
    ($pool:expr, $n:expr) => {{
        cdr!(car!(car!(car!($pool, $n))))
    }}
}

macro_rules! cdaadr {
    ($pool:expr, $n:expr) => {{
        cdr!(car!(car!(cdr!($pool, $n))))
    }}
}

macro_rules! cdadar {
    ($pool:expr, $n:expr) => {{
        cdr!(car!(cdr!(cdr!($pool, $n))))
    }}
}

macro_rules! cdaddr {
    ($pool:expr, $n:expr) => {{
        cdr!(car!(cdr!(cdr!($pool, $n))))
    }}
}

macro_rules! cddaar {
    ($pool:expr, $n:expr) => {{
        cdr!(cdr!(car!(car!($pool, $n))))
    }}
}

macro_rules! cddadr {
    ($pool:expr, $n:expr) => {{
        cdr!(cdr!(car!(cdr!($pool, $n))))
    }}
}

macro_rules! cdddar {
    ($pool:expr, $n:expr) => {{
        cdr!(cdr!(cdr!(car!($pool, $n))))
    }}
}

macro_rules! cddddr {
    ($pool:expr, $n:expr) => {{
        cdr!(cdr!(cdr!(cdr!($pool, $n))))
    }}
}

/*
 * Tagged data types
 */

const T_BYTECODE: i32 = -10;
const T_CATCHTAG: i32 = -11;
const T_CHAR:     i32 = -12;
const T_CLOSURE:  i32 = -13;
const T_FIXNUM:   i32 = -14;
const T_INPORT:   i32 = -15;
const T_OUTPORT:  i32 = -16;
const T_STRING:   i32 = -17;
const T_SYMBOL:   i32 = -18;
const T_VECTOR:   i32 = -19;

/*
 * Basic constructors
 */

macro_rules! cons {
    ($a:expr, $d:expr) => {{
        cons3($a, $d, 0);
    }}
}

macro_rules! mkatom {
    ($a:expr, $d:expr) => {{
        cons3($a, $d, ATOM_TAG);
    }}
}

/*
 * Accessors
 */

macro_rules! portno {
    ($pool:expr, $n:expr) => {{
        cadr!($pool, $n);
    }}
}

// this likely won't work. in the C version,
// this macro returns a pointer to the string,
// but we won't be using pointers.
// for now we'll just return n
macro_rules! string {
    ($vpool:expr, $n:expr) => {{
        $vpool[cdr!($vpool, $n)];
    }}
}

macro_rules! stringlen {
    ($vpool:expr, $n:expr) => {{
        let index = $n - 2 as usize;
        $vpool[cdr!($vpool, index)];
    }}
}

macro_rules! symname {
    ($vpool:expr, $n:expr) => {{
        string!($vpool, $n);
    }}
}

macro_rules! vector {
    ($vpool:expr, $n:expr) => {{
        let index = $n as usize;
        $vpool[index];
    }}
}

macro_rules! veclink {
    ($vpool:expr, $n:expr) => {{
        let index = $n as usize;
        $vpool[index - 2];
    }}
}

macro_rules! vecndx {
    ($vpool:expr, $n:expr) => {{
        veclink!($vpool, $n);
    }}
}

// this also differs from the C version, since Rust handles
// sizing of the vectors for us.
macro_rules! vecsize {
    ($k:expr) => {{
        2 + $k
    }}
}

macro_rules! veclen {
    ($vpool:expr, $n:expr) => {{
        vecsize!(stringlen!($vpool, $n));
    }}
}

/*
 * Type predicates
 */

macro_rules! charp {
    ($pool:expr, $n:expr) => {{
        !specialp!() && (tag!($pool, $n) & ATOM_TAG) && T_CHAR == car($pool, $n);
    }}
}

macro_rules! closurep {
    ($pool:expr, $n:expr) => {{
        !specialp!(n) && (tag!($pool, $n) & ATOM_TAG) && T_CLOSURE == car($pool, $n);
    }}
}

fn alloc_nodepool() -> NodePool {
    let mut cars: Car = Vec::with_capacity(NNODES);
    let mut cdrs: Cdr = Vec::with_capacity(NNODES);
    let mut tags: Tag = Vec::with_capacity(NNODES);

    cars.push(0);
    cdrs.push(0);
    tags.push(ATOM_TAG);

    NodePool{ cars, cdrs, tags }
}

fn main () {
    /*
     * Memory pool allocation.
     */
    let nodepool: NodePool = alloc_nodepool();

    println!("Car: {}", car!(nodepool, 0));
    println!("Cdr: {}", cdr!(nodepool, 0));
    println!("Tag: {}", tag!(nodepool, 0));
}
