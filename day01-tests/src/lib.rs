use day01_core::gradients;
use day01_core::sum;

#[test]
fn gradient_returns_empty_for_one()
{
	let input = vec![10];

	let output = gradients(&input);

	let expected = vec![];
	assert_eq!(&output, &expected);
}

#[test]
fn gradient_returns_single_for_two()
{
	let input = vec![10, 12];

	let output = gradients(&input);

	let expected = vec![2];
	assert_eq!(&output, &expected);
}

#[test]
fn gradient_returns_two_for_three()
{
	let input = vec![10, 15, 5];

	let output = gradients(&input);

	let expected = vec![5, -10];
	assert_eq!(&output, &expected);
}

#[test]
fn sum_returns_one_for_one_positive()
{
	let gradients = vec![5, -5];

	let output = sum(&gradients);

	assert_eq!(output, 1);
}

#[test]
fn sum_returns_two_for_two_positive()
{
	let gradients = vec![5, -5, -8, 3, -11];

	let output = sum(&gradients);

	assert_eq!(output, 2);
}

#[test]
fn gradient_example()
{
	let input = vec![
		199,
		200,
		208,
		210,
		200,
		207,
		240,
		269,
		260,
		263,
	];

	let output = gradients(&input);

	let expected = vec![
		1,
		8,
		2,
		-10,
		7,
		33,
		29,
		-9,
		3,
	];
	assert_eq!(&output, &expected);
}

#[test]
fn example()
{
	let input = vec![
		199,
		200,
		208,
		210,
		200,
		207,
		240,
		269,
		260,
		263,
	];

	let gradients = gradients(&input);
	let sum = sum(&gradients);

	assert_eq!(sum, 7);
}
