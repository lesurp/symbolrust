use ndarray::*;
use symbolrust::prelude::*;

fn main() {
    let mut a = Array2::<Node>::zeros((2, 2));
    let x = Variable::new(0);
    a[(0, 1)] = x.into();
    let b = Array2::<Variable>::from_shape_fn((2, 2), |(i, j)| Variable::new(i + j * 2 + 11));
    let mut pp_context = PrettyPrinterContext::new();

    //  TODO: abstract this somehow
    b.indexed_iter()
        .for_each(|((i, j), var)| pp_context.name_var(*var, format!("b_{}_{}", i, j)));

    let c = (a + b).map(ConstantFolder::fold);

    let mut context = Context::new();
    context.assign(x, 23.into());
    let d = c.map(|node| Evaluator::evaluate(node, &context));

    println!(
        "{}",
        PrettyPrinter::print_array_with_context(&c, &pp_context)
    );
    pp_context.name_var(x, "x");
    println!(
        "{}",
        PrettyPrinter::print_array_with_context(&c, &pp_context)
    );
    println!(
        "{}",
        PrettyPrinter::print_array_with_context(&d, &pp_context)
    );
}
