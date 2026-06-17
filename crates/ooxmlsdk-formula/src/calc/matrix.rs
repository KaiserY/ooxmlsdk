use super::numeric::KahanSum;

#[derive(Clone, Debug, PartialEq)]
pub struct LupDecomposition {
  pub permutation: Vec<usize>,
  pub determinant_sign: f64,
}

pub fn qr_decompose(matrix: &mut [Vec<f64>], k: usize, n: usize) -> Option<Vec<f64>> {
  let mut r_diagonal = vec![0.0; k];
  for column in 0..k {
    let scale = (column..n)
      .map(|row| matrix[row][column].abs())
      .fold(0.0, f64::max);
    if scale == 0.0 {
      return None;
    }
    for row_values in matrix.iter_mut().take(n).skip(column) {
      row_values[column] /= scale;
    }
    let euclid = kahan_product_sum(
      column..n,
      |row| matrix[row][column],
      |row| matrix[row][column],
    )
    .sqrt();
    let factor = 1.0 / euclid / (euclid + matrix[column][column].abs());
    let sign = if matrix[column][column] >= 0.0 {
      1.0
    } else {
      -1.0
    };
    matrix[column][column] += sign * euclid;
    r_diagonal[column] = -sign * scale * euclid;
    for c in column + 1..k {
      let sum = kahan_product_sum(column..n, |row| matrix[row][column], |row| matrix[row][c]);
      for row_values in matrix.iter_mut().take(n).skip(column) {
        row_values[c] -= sum * factor * row_values[column];
      }
    }
  }
  Some(r_diagonal)
}

pub fn apply_householder(matrix: &[Vec<f64>], column: usize, values: &mut [f64], n: usize) {
  let denominator = kahan_product_sum(
    column..n,
    |row| matrix[row][column],
    |row| matrix[row][column],
  );
  let numerator = kahan_product_sum(column..n, |row| matrix[row][column], |row| values[row]);
  let factor = if denominator == 0.0 {
    0.0
  } else {
    2.0 * numerator / denominator
  };
  for row in column..n {
    values[row] -= factor * matrix[row][column];
  }
}

pub fn solve_upper(matrix: &[Vec<f64>], diagonal: &[f64], values: &mut [f64], k: usize) {
  for row in (0..k).rev() {
    let mut sum = KahanSum::default();
    sum.add(values[row]);
    for column in row + 1..k {
      sum.add(-matrix[row][column] * values[column]);
    }
    values[row] = sum.finish() / diagonal[row];
  }
}

pub fn solve_lower(matrix: &[Vec<f64>], diagonal: &[f64], values: &mut [f64], k: usize) {
  for row in 0..k {
    let mut sum = KahanSum::default();
    sum.add(values[row]);
    for column in 0..row {
      sum.add(-matrix[column][row] * values[column]);
    }
    values[row] = sum.finish() / diagonal[row];
  }
}

pub fn determinant(mut matrix: Vec<Vec<f64>>) -> f64 {
  let n = matrix.len();
  if matrix.iter().any(|row| row.len() != n) {
    return 0.0;
  }
  let Some(decomposition) = lup_decompose(&mut matrix) else {
    return 0.0;
  };
  let mut det = decomposition.determinant_sign;
  for (index, row) in matrix.iter().enumerate() {
    det *= row[index];
  }
  det
}

pub fn matrix_multiply(left: &[Vec<f64>], right: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
  let rows = left.len();
  let shared = left.first().map_or(0, Vec::len);
  let right_rows = right.len();
  let columns = right.first().map_or(0, Vec::len);
  if rows == 0
    || shared == 0
    || right_rows == 0
    || columns == 0
    || left.iter().any(|row| row.len() != shared)
    || right.iter().any(|row| row.len() != columns)
    || shared != right_rows
  {
    return None;
  }

  let mut result = Vec::with_capacity(rows);
  for left_row in left {
    let mut result_row = Vec::with_capacity(columns);
    for (column, _) in right[0].iter().enumerate().take(columns) {
      result_row.push(kahan_product_sum(
        0..shared,
        |index| left_row[index],
        |index| right[index][column],
      ));
    }
    result.push(result_row);
  }
  Some(result)
}

pub fn lup_decompose(matrix: &mut [Vec<f64>]) -> Option<LupDecomposition> {
  let n = matrix.len();
  if matrix.iter().any(|row| row.len() != n) {
    return None;
  }
  let mut determinant_sign = 1.0;
  let mut scale = vec![0.0; n];
  for (row, matrix_row) in matrix.iter().enumerate().take(n) {
    let mut max = 0.0;
    for value in matrix_row.iter().take(n) {
      let value = value.abs();
      if max < value {
        max = value;
      }
    }
    if max == 0.0 {
      return None;
    }
    scale[row] = 1.0 / max;
  }

  let mut permutation = (0..n).collect::<Vec<_>>();
  for pivot in 0..n.saturating_sub(1) {
    let mut max = 0.0;
    let mut pivot_row = pivot;
    let pivot_scale = scale[pivot];
    for (row, matrix_row) in matrix.iter().enumerate().take(n).skip(pivot) {
      let value = pivot_scale * matrix_row[pivot].abs();
      if max < value {
        max = value;
        pivot_row = row;
      }
    }
    if max == 0.0 {
      return None;
    }
    if pivot != pivot_row {
      permutation.swap(pivot, pivot_row);
      scale.swap(pivot, pivot_row);
      matrix.swap(pivot, pivot_row);
      determinant_sign = -determinant_sign;
    }
    let (head, tail) = matrix.split_at_mut(pivot + 1);
    let pivot_row = &head[pivot];
    let denominator = pivot_row[pivot];
    for row_values in tail.iter_mut().take(n - pivot - 1) {
      let numerator = row_values[pivot];
      row_values[pivot] = numerator / denominator;
      for (column, value) in row_values.iter_mut().enumerate().take(n).skip(pivot + 1) {
        *value = (*value * denominator - numerator * pivot_row[column]) / denominator;
      }
    }
  }

  if (0..n).any(|index| matrix[index][index] == 0.0) {
    None
  } else {
    Some(LupDecomposition {
      permutation,
      determinant_sign,
    })
  }
}

pub fn lup_solve(
  lu: &[Vec<f64>],
  decomposition: &LupDecomposition,
  rhs: &[f64],
) -> Option<Vec<f64>> {
  let n = lu.len();
  if rhs.len() != n
    || decomposition.permutation.len() != n
    || lu.iter().any(|row| row.len() != n)
    || decomposition.permutation.iter().any(|&row| row >= n)
  {
    return None;
  }
  let mut x = vec![0.0; n];
  let mut first_nonzero = None;
  for row in 0..n {
    let mut sum = KahanSum::default();
    sum.add(rhs[decomposition.permutation[row]]);
    if let Some(first) = first_nonzero {
      for column in first..row {
        sum.add(-lu[row][column] * x[column]);
      }
    } else if rhs[decomposition.permutation[row]] != 0.0 {
      first_nonzero = Some(row);
    }
    x[row] = sum.finish();
  }
  for row in (0..n).rev() {
    let mut sum = KahanSum::default();
    sum.add(x[row]);
    for column in (row + 1)..n {
      sum.add(-lu[row][column] * x[column]);
    }
    x[row] = sum.finish() / lu[row][row];
  }
  Some(x)
}

fn kahan_product_sum(
  range: impl IntoIterator<Item = usize>,
  left: impl Fn(usize) -> f64,
  right: impl Fn(usize) -> f64,
) -> f64 {
  let mut sum = KahanSum::default();
  for index in range {
    sum.add(left(index) * right(index));
  }
  sum.finish()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn determinant_uses_lup_pivot_sign() {
    let matrix = vec![vec![0.0, 2.0], vec![1.0, 3.0]];
    assert_eq!(determinant(matrix), -2.0);
  }

  #[test]
  fn determinant_and_lup_reject_ragged_matrix() {
    assert_eq!(determinant(vec![vec![1.0, 2.0], vec![3.0]]), 0.0);

    let mut matrix = vec![vec![1.0, 2.0], vec![3.0]];
    assert!(lup_decompose(&mut matrix).is_none());
  }

  #[test]
  fn matrix_multiply_rejects_bad_shapes_and_multiplies() {
    assert_eq!(
      matrix_multiply(
        &[vec![1.0, 2.0], vec![3.0, 4.0]],
        &[vec![5.0, 6.0], vec![7.0, 8.0]]
      ),
      Some(vec![vec![19.0, 22.0], vec![43.0, 50.0]])
    );
    assert!(matrix_multiply(&[vec![1.0, 2.0]], &[vec![1.0, 2.0]]).is_none());
  }

  #[test]
  fn lup_decompose_and_solve_linear_system() {
    let mut matrix = vec![vec![4.0, 7.0], vec![2.0, 6.0]];
    let decomposition = lup_decompose(&mut matrix).unwrap();
    let solution = lup_solve(&matrix, &decomposition, &[1.0, 0.0]).unwrap();
    assert!((solution[0] - 0.6).abs() < 1.0e-12);
    assert!((solution[1] + 0.2).abs() < 1.0e-12);
  }

  #[test]
  fn lup_solve_rejects_incompatible_dimensions() {
    let mut matrix = vec![vec![4.0, 7.0], vec![2.0, 6.0]];
    let decomposition = lup_decompose(&mut matrix).unwrap();
    assert!(lup_solve(&matrix, &decomposition, &[1.0]).is_none());
  }

  #[test]
  fn qr_solves_overdetermined_system() {
    let mut matrix = vec![vec![1.0], vec![2.0], vec![3.0]];
    let diagonal = qr_decompose(&mut matrix, 1, 3).unwrap();
    let mut values = vec![2.0, 4.0, 6.0];
    apply_householder(&matrix, 0, &mut values, 3);
    solve_upper(&matrix, &diagonal, &mut values, 1);
    assert!((values[0] - 2.0).abs() < 1.0e-12);
  }
}
