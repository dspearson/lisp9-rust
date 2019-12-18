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
const NVCELLS:    usize = 262144;
const NPORTS:     i32   = 20;
const TOKLEN:     i32   = 80;
const CHUNKSIZE:  i32   = 1024;
const MXMAX:      i32   = 2000;
const NTRACE:     usize = 10;
const PRDEPTH:    i32   = 1024;

/*
 * Basic data types
 */

type Cell = i32;
type Byte = char;
type Uint = u32;

/*
 * Special Objects
 */

macro_rules! specialp {
    ($x:expr) => {{
        $x < 0
    }}
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
        let tags = &$pool.tags;
        let index = $n as usize;
        tags[index]
    }}
}

macro_rules! car {
    ($pool:expr, $n:expr) => {{
        let pool: &NodePool = $pool;
        let cars = &pool.cars;
        let index = $n as usize;
        cars[index]
    }}
}

macro_rules! cdr {
    ($pool:expr, $n:expr) => {{
        let pool: &NodePool = $pool;
        let cdrs = &pool.cdrs;
        let index = $n as usize;
        cdrs[index]
    }}
}

macro_rules! caar {
    ($pool:expr, $n:expr) => {{
        car!($pool, car!($pool, $n))
    }}
}

macro_rules! cadr {
    ($pool:expr, $n:expr) => {{
        car!($pool, cdr!($pool, $n))
    }}
}

macro_rules! cdar {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, car!($pool, $n))
    }}
}

macro_rules! cddr {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, cdr!($pool, $n))
    }}
}

macro_rules! caaar {
    ($pool:expr, $n:expr) => {{
        car!($pool, car!($pool, (car!($pool, $n))))
    }}
}

macro_rules! caadr {
    ($pool:expr, $n:expr) => {{
        car!($pool, car!($pool, cdr!($pool, $n)))
    }}
}

macro_rules! cadar {
    ($pool:expr, $n:expr) => {{
        car!($pool, cdr!($pool, car!($pool, $n)))
    }}
}

macro_rules! caddr {
    ($pool:expr, $n:expr) => {{
        car!($pool, car!($pool, cdr!($pool, $n)))
    }}
}

macro_rules! cdaar {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, car!($pool, car!($pool, $n)))
    }}
}

macro_rules! cdadr {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, car!($pool, cdr!($pool, $n)))
    }}
}

macro_rules! cddar {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, cdr!($pool, car!($pool, $n)))
    }}
}

macro_rules! cdddr {
    ($pool:expr, $n:expr) => {{
        car!($pool, cdr!($pool, cdr!($pool, $n)))
    }}
}

macro_rules! caaaar {
    ($pool:expr, $n:expr) => {{
        car!($pool, car!($pool, car!($pool, car!($pool, $n))))
    }}
}

macro_rules! caaadr {
    ($pool:expr, $n:expr) => {{
        car!($pool, car!($pool, car!($pool, cdr!($pool, $n))))
    }}
}

macro_rules! caadar {
    ($pool:expr, $n:expr) => {{
        car!($pool, car!($pool, cdr!($pool, car!($pool, $n))))
    }}
}

macro_rules! caaddr {
    ($pool:expr, $n:expr) => {{
        car!($pool, car!($pool, cdr!($pool, cdr!($pool, $n))))
    }}
}

macro_rules! cadaar {
    ($pool:expr, $n:expr) => {{
        car!($pool, cdr!($pool, car!($pool, car!($pool, $n))))
    }}
}

macro_rules! cadadr {
    ($pool:expr, $n:expr) => {{
        car!($pool, cdr!($pool, car!($pool, cdr!($pool, $n))))
    }}
}

macro_rules! caddar {
    ($pool:expr, $n:expr) => {{
        car!($pool, cdr!($pool, cdr!($pool, car!($pool, $n))))
    }}
}

macro_rules! cadddr {
    ($pool:expr, $n:expr) => {{
        car!($pool, cdr!($pool, cdr!($pool, cdr!($pool, $n))))
    }}
}

macro_rules! cdaaar {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, car!($pool, car!($pool, car!($pool, $n))))
    }}
}

macro_rules! cdaadr {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, car!($pool, car!($pool, cdr!($pool, $n))))
    }}
}

macro_rules! cdadar {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, car!($pool, cdr!($pool, cdr!($pool, $n))))
    }}
}

macro_rules! cdaddr {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, car!($pool, cdr!($pool, cdr!($pool, $n))))
    }}
}

macro_rules! cddaar {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, cdr!($pool, car!($pool, car!($pool, $n))))
    }}
}

macro_rules! cddadr {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, cdr!($pool, car!($pool, cdr!($pool, $n))))
    }}
}

macro_rules! cdddar {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, cdr!($pool, cdr!($pool, car!($pool, $n))))
    }}
}

macro_rules! cddddr {
    ($pool:expr, $n:expr) => {{
        cdr!($pool, cdr!($pool, cdr!($pool, cdr!($pool, $n))))
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
        $vpool[cdr!($vpool, $n)]
    }}
}

macro_rules! stringlen {
    ($vpool:expr, $n:expr) => {{
        let index = $n - 2 as usize;
        $vpool[cdr!($vpool, index)]
    }}
}

macro_rules! symname {
    ($vpool:expr, $n:expr) => {{
        string!($vpool, $n)
    }}
}

macro_rules! vector {
    ($vpool:expr, $n:expr) => {{
        let index = $n as usize;
        $vpool[index]
    }}
}

macro_rules! veclink {
    ($vpool:expr, $n:expr) => {{
        let index = $n as usize;
        $vpool[index - 2]
    }}
}

macro_rules! vecndx {
    ($vpool:expr, $n:expr) => {{
        veclink!($vpool, $n)
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
        vecsize!(stringlen!($vpool, $n))
    }}
}

/*
 * Type predicates
 */

macro_rules! charp {
    ($pool:expr, $n:expr) => {{
        !specialp!($n) && (tag!($pool, $n) & ATOM_TAG == 1) && T_CHAR == car!($pool, $n)
    }}
}

macro_rules! closurep {
    ($pool:expr, $n:expr) => {{
        !specialp!($n) && (tag!($pool, $n) & ATOM_TAG == 1) && T_CLOSURE == car!($pool, $n)
    }}
}

macro_rules! ctagp {
    ($pool:expr, $n:expr) => {{
        !specialp!($n) && (tag($pool, $n) & ATOM_TAG == 1) && T_CATCHTAG == car!($pool, $n)
    }}
}

macro_rules! eofp {
    ($n:expr) => {{
        EOFMARK == $n
    }}
}

macro_rules! fixp {
    ($pool:expr, $n:expr) => {{
        !specialp!($n) && (tag!($pool, $n) & ATOM_TAG == 1) && T_FIXNUM == car!($pool, $n)
    }}
}

macro_rules! inport {
    ($pool:expr, $n:expr) => {{
        !specialp!($n) && (tag!($pool, $n) & ATOM_TAG == 1) && (tag!($pool, $n) & PORT_TAG == 1) && T_INPORT == car!($pool, $n)
    }}
}

macro_rules! outport {
    ($pool:expr, $n:expr) => {{
        !specialp!($n) && (tag!($pool, $n) & ATOM_TAG == 1) && (tag!($pool, $n) & PORT_TAG == 1) && T_OUTPORT == car!($pool, $n)
    }}
}

macro_rules! stringp {
    ($pool:expr, $n:expr) => {{
        !specialp!($n) && (tag!($pool, $n) & VECTOR_TAG == 1) && T_STRING == car!($pool, $n)
    }}
}

macro_rules! symbolp {
    ($pool:expr, $n:expr) => {{
        !specialp!($n) && (tag!($pool, $n) & VECTOR_TAG == 1) && T_SYMBOL == car!($pool, $n)
    }}
}

macro_rules! vectorp {
    ($pool:expr, $n:expr) => {{
        !specialp!($n) && (tag!($pool, $n) & VECTOR_TAG == 1) && T_VECTOR == car!($pool, $n)
    }}
}

macro_rules! atomp {
    ($pool:expr, $n:expr) => {{
        specialp!($n) || (tag!($pool, $n) & ATOM_TAG == 1) || (tag!($pool, $n) & VECTOR_TAG == 1)
    }}
}

macro_rules! pairp {
    ($pool:expr, $x:expr) => {{
        !atomp!($pool, $x)
    }}
}

macro_rules! listp {
    ($pool:expr, $x:expr) => {{
        NIL == $x || pairp!($pool, $x)
    }}
}

macro_rules! constp {
    ($pool:expr, $n:expr) => {{
        !specialp!($n) && (tag!($pool, $n) & CONST_TAG == 1)
    }}
}

/*
 * Abstract machine opcodes
 */

enum OpCodes {
    OpIll, OpApplis, OpApplist, OpApply, OpTailapp, OpQuote, OpArg, OpRef, OpPush,
    OpPushtrue, OpPushval, OpPop, OpDrop, OpJmp, OpBrf, OpBrt, OpHalt, OpCatchstar,
    OpThrowstar, OpClosure, OpMkenv, OpPropenv, OpCpref, OpCparg, OpEnter, OpEntcol,
    OpReturn, OpSetarg, OpSetref, OpMacro,

    OpAbs, OpAlphac, OpAtom, OpBitop, OpCaar, OpCadr, OpCar, OpCdar, OpCddr, OpCdr,
    OpCequal, OpCgrtr, OpCgteq, OpChar, OpCharp, OpCharval, OpCless, OpClosePort,
    OpClteq, OpCmdline, OpConc, OpCons, OpConstp, OpCtagp, OpDelete, OpDiv,
    OpDowncase, OpDumpImage, OpEofp, OpEq, OpEqual, OpError, OpError2, OpErrport,
    OpEval, OpExistsp, OpFixp, OpFormat, OpFunp, OpGc, OpGensym, OpGrtr, OpGteq,
    OpInport, OpInportp, OpLess, OpListstr, OpListvec, OpLoad, OpLowerc, OpLteq,
    OpMax, OpMin, OpMinus, OpMkstr, OpMkvec, OpMx, OpMx1, OpNconc, OpNegate,
    OpNreconc, OpNull, OpNumeric, OpNumstr, OpObtab, OpOpenInfile, OpOpenOutfile,
    OpOutport, OpOutportp, OpPair, OpPeekc, OpPlus, OpPrin, OpPrinc, OpQuit, OpRead,
    OpReadc, OpReconc, OpRem, OpSconc, OpSequal, OpSetcar, OpSetcdr, OpSetInport,
    OpSetOutport, OpSfill, OpSgrtr, OpSgteq, OpSiequal, OpSigrtr, OpSigteq,
    OpSiless, OpSilteq, OpSless, OpSlteq, OpSref, OpSset, OpSsize, OpStringp,
    OpStrlist, OpStrnum, OpSubstr, OpSubvec, OpSymbol, OpSymbolp, OpSymname,
    OpSymtab, OpSyscmd, OpTimes, OpUntag, OpUpcase, OpUpperc, OpVconc, OpVeclist,
    OpVectorp, OpVfill, OpVref, OpVset, OpVsize, OpWhitec, OpWritec,
}

/*
 * I/O Functions
 */

macro_rules! printb {
    ($s:expr) => {{
        prints($s)
    }}
}

macro_rules! nl {
    () => {{
        prints("\n")
    }}
}

/*
 * Error reporting and handling
 */

type TraceRing = Vec<i32>;
type TracePointer = i32;

struct Trace {
    trace: TraceRing,
    p: TracePointer
}

// int Trace[NTRACE];
// int Tp = 0;

fn alloc_tracevec() -> Trace {
    let mut trace: TraceRing = Vec::with_capacity(NTRACE);
    let p: TracePointer = 0;

    for _i in 0 .. NTRACE {
        trace.push(-1);
    }

    Trace { trace, p }
}

fn clrtrace(trace: &mut Trace) {
    for i in 0 .. NTRACE {
        trace.trace[i] = -1;
    }
}

fn gottrace(trace: &Trace) -> bool {
    for i in 0 .. NTRACE {
        if trace.trace[i] != -1 {
            return true;
        }
    }
    false
}

// int Plimit = 0;
// int Line = 1;
// cell Files = NIL;
// cell Symbols;

// Plimit should probably be thrown somewhere else.
// it's a global, mutable state variable for limiting
// printer output.

// fn report(files: &Vec<Cell>, s: &Vec<Cell>, x: Cell) {
//     let o = set_outport(2);
//     prints("*** error: ");
//     prints(s);
//     if x != UNDEF {
//         prints(": ");
//         let mut Plimit = 100;
//         prin(x);
//         let mut Plimit = 0;
//     }
//     nl!();
//     if files != NIL {
//         prints("*** file: ");
//         printb!(string!(files, car!(files, 0)));
//     }
// }

// noop
fn report(s: &Vec<Cell>, y: Cell) -> Cell {
    0
}

// noop
fn bindset(x: Cell, y: Vec<Cell>) -> Cell {
    x
}

// noop
fn assq(x: Cell, y: Cell) -> Cell {
    x
}

// noop
fn mkstr(x: &Vec<Cell>, l: i32) -> Vec<Cell> {
    Vec::with_capacity(0)
}

// noop
fn strlen(s: &Vec<Cell>) -> i32 {
    0
}

// noop
fn longjmp(tag: i32, n: i32) {
}

// Rust, I think, has no good concept of jump buffers, so this entire
// set of control flow mechanics may need to be thrown out the window.
// type JmpBuf

// needs rewriting, uses C concepts.
fn error(pool: &NodePool, s: &Vec<Cell>, x: Cell, s_errtag: Cell, s_errval: Cell, glob: Cell) {
    let n: Cell = assq(s_errtag, glob);
    let handler: Cell = if NIL == n { NIL } else { cadr!(pool, n) };
    if handler != NIL {
        let n: Cell = assq(s_errval, glob);
        if n != NIL && cadr!(pool, n) == handler {
            bindset(s_errval, mkstr(s, strlen(s)));
        }
        longjmp(0, 1); // probably needs refactoring, longjmp in rust is not good.
    }
    report(s, x);
    longjmp(0, 1);
}

// fn expect(pool: &Vec<Cell>, who: &Vec<Cell>, what: &Vec<Cell>, got: Cell) {
//     let b: Vec<Cell> = Vec::with_capacity(100);

//     sprintf(&b, "%s: expected %s", &who, &what);
//     error(pool, &b, got, 0, 0, 0);
// }

// exit
fn fatal(s: char) {
    0;
}

fn readc() -> i32 {
    return 0;
}

fn alloc_nodepool() -> NodePool {
    let mut cars: Car = Vec::with_capacity(NNODES);
    let mut cdrs: Cdr = Vec::with_capacity(NNODES);
    let mut tags: Tag = Vec::with_capacity(NNODES);

    for _i in 0 .. NNODES {
        cars.push(0);
        cdrs.push(0);
        tags.push(0);
    }

    NodePool{ cars, cdrs, tags }
}

fn alloc_vecpool() -> Vectors {
    let mut vectors: Vectors = Vec::with_capacity(NVCELLS);

    for _i in 0 .. NVCELLS {
        vectors.push(0);
    }

    vectors
}

const OBFREE:  i32 = 0;
const OBALLOC: i32 = 1;
const OBUSED:  i32 = 2;

const ISIZE0:  i32 = 1;
const ISIZE1:  i32 = 3;
const ISIZE2:  i32 = 5;

fn main() {
    /*
     * Memory pool allocation.
     */
    let nodepool: NodePool = alloc_nodepool();

    /*
     * Trace structure allocation.
     */
    let mut trace = alloc_tracevec();

    let mut files: Vec<Cell> = Vec::with_capacity(NTRACE);

    println!("Trace vector position 0 is -1? {}", trace.trace[0] == -1);
    println!("Got a trace? {}", gottrace(&trace));
    println!("Car: {}", car!(&nodepool, 0));
    println!("Cdr: {}", cdr!(&nodepool, 0));
    println!("Tag: {}", tag!(&nodepool, 0));
    println!("Special? {}", specialp!(-1));
    println!("EOFMARK? {}", eofp!(EOFMARK));
    println!("Bitwise AND, {}", ATOM_TAG & ATOM_TAG == 1);
    println!("List? {}", listp!(nodepool, NIL));
    println!("Constant? {}", constp!(nodepool, 0));
}
