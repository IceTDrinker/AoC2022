use super::load_file;

/// --- Day 8: Treetop Tree House ---
/// The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The
/// Elves explain that a previous expedition planted these trees as a reforestation effort. Now,
/// they're curious if this would be a good location for a tree house.
///
/// First, determine whether there is enough tree cover here to keep a tree house hidden. To do
/// this, you need to count the number of trees that are visible from outside the grid when looking
/// directly along a row or column.
///
/// The Elves have already launched a quadcopter to generate a map with the height of each tree
/// (your puzzle input). For example:
///
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
/// Each tree is represented as a single digit whose value is its height, where 0 is the shortest
/// and 9 is the tallest.
///
/// A tree is visible if all of the other trees between it and an edge of the grid are shorter than
/// it. Only consider trees in the same row or column; that is, only look up, down, left, or right
/// from any given tree.
///
/// All of the trees around the edge of the grid are visible - since they are already on the edge,
/// there are no trees to block the view. In this example, that only leaves the interior nine trees
/// to consider:
///
/// The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom
/// since other trees of height 5 are in the way.) The top-middle 5 is visible from the top and
/// right. The top-right 1 is not visible from any direction; for it to be visible, there would need
/// to only be trees of height 0 between it and an edge. The left-middle 5 is visible, but only from
/// the right. The center 3 is not visible from any direction; for it to be visible, there would
/// need to be only trees of at most height 2 between it and an edge. The right-middle 3 is visible
/// from the right. In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
/// With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are
/// visible in this arrangement.
///
/// Consider your map; how many trees are visible from outside the grid?
///
/// --- Part Two ---
/// Content with the amount of tree cover available, the Elves just need to know the best spot to
/// build their tree house: they would like to be able to see a lot of trees.
///
/// To measure the viewing distance from a given tree, look up, down, left, and right from that
/// tree; stop if you reach an edge or at the first tree that is the same height or taller than the
/// tree under consideration. (If a tree is right on the edge, at least one of its viewing distances
/// will be zero.)
///
/// The Elves don't care about distant trees taller than those found by the rules above; the
/// proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than
/// the tree house anyway.
///
/// In the example above, consider the middle 5 in the second row:
///
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
/// Looking up, its view is not blocked; it can see 1 tree (of height 3).
/// Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next
/// to it). Looking right, its view is not blocked; it can see 2 trees.
/// Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree
/// of height 5 that blocks its view). A tree's scenic score is found by multiplying together its
/// viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying
/// 1 * 1 * 2 * 2).
///
/// However, you can do even better: consider the tree of height 5 in the middle of the fourth row:
///
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
/// Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
/// Looking left, its view is not blocked; it can see 2 trees.
/// Looking down, its view is also not blocked; it can see 1 tree.
/// Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
/// This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.
///
/// Consider each tree on your map. What is the highest scenic score possible for any tree?
pub fn day_08() {
    let data = load_file(8);
    let mut data_as_lines = data.trim().split('\n').peekable();

    let line_len = data_as_lines.peek().unwrap().len();
    let line_count = data_as_lines.clone().count();

    let mut forest = vec![0u32; line_len * line_count];
    for (line, tree_line) in data_as_lines.zip(forest.chunks_mut(line_len)) {
        tree_line
            .iter_mut()
            .zip(line.chars())
            .for_each(|(dst, src)| *dst = src.to_digit(10).unwrap());
    }

    let forest = forest;

    let mut visible_forest = vec![0u32; line_len * line_count];

    for (tree_line, visibility_line) in forest
        .chunks(line_len)
        .zip(visible_forest.chunks_mut(line_len))
    {
        let mut must_be_higher_to_be_seen_from_left = tree_line[0];
        visibility_line
            .iter_mut()
            .zip(tree_line.iter())
            .for_each(|(visibility, &tree_height)| {
                if tree_height > must_be_higher_to_be_seen_from_left {
                    must_be_higher_to_be_seen_from_left = tree_height;
                    *visibility |= 1;
                }
            });
        visibility_line[0] |= 1;

        let mut must_be_higher_to_be_seen_from_right = *tree_line.last().unwrap();

        visibility_line
            .iter_mut()
            .zip(tree_line.iter())
            .rev()
            .for_each(|(visibility, &tree_height)| {
                if tree_height > must_be_higher_to_be_seen_from_right {
                    must_be_higher_to_be_seen_from_right = tree_height;
                    *visibility |= 1;
                }
            });

        let right_tree = visibility_line.last_mut().unwrap();
        *right_tree |= 1;
    }

    for column_idx in 0..line_len {
        let mut tree_column_iter = forest.iter().skip(column_idx).step_by(line_len);
        let mut visibility_column_iter =
            visible_forest.iter_mut().skip(column_idx).step_by(line_len);

        let mut must_be_higher_to_be_seen_from_top = *tree_column_iter.next().unwrap();
        let top_tree_visible = visibility_column_iter.next().unwrap();
        *top_tree_visible |= 1;

        visibility_column_iter
            .zip(tree_column_iter)
            .for_each(|(visibility, &tree_height)| {
                if tree_height > must_be_higher_to_be_seen_from_top {
                    must_be_higher_to_be_seen_from_top = tree_height;
                    *visibility |= 1;
                }
            });

        let mut tree_column_iter = forest.iter().skip(column_idx).step_by(line_len).rev();
        let mut visibility_column_iter = visible_forest
            .iter_mut()
            .skip(column_idx)
            .step_by(line_len)
            .rev();

        let mut must_be_higher_to_be_seen_from_bot = *tree_column_iter.next().unwrap();
        let bot_tree_visible = visibility_column_iter.next().unwrap();
        *bot_tree_visible |= 1;

        visibility_column_iter
            .zip(tree_column_iter)
            .for_each(|(visibility, &tree_height)| {
                if tree_height > must_be_higher_to_be_seen_from_bot {
                    must_be_higher_to_be_seen_from_bot = tree_height;
                    *visibility |= 1;
                }
            });
    }

    let visible_tree_count: u32 = visible_forest.iter().sum();

    println!("Part 1: {visible_tree_count}");

    let mut scenic_score = vec![1u32; forest.len()];

    for ((tree_idx, &tree_height), scenic_score) in
        forest.iter().enumerate().zip(scenic_score.iter_mut())
    {
        let column_idx = tree_idx % line_len;

        let mut right_viewing_distance = 0;
        let right_view_iter = forest
            .iter()
            .skip(tree_idx + 1)
            .take(line_len - (column_idx + 1));
        for &other_tree_height in right_view_iter {
            right_viewing_distance += 1;
            if tree_height <= other_tree_height {
                break;
            }
        }
        *scenic_score *= right_viewing_distance;

        let mut left_viewing_distance = 0;
        let left_view_iter = forest
            .iter()
            .rev()
            .skip(forest.len() - tree_idx)
            .take(column_idx);
        for &other_tree_height in left_view_iter {
            left_viewing_distance += 1;
            if tree_height <= other_tree_height {
                break;
            }
        }
        *scenic_score *= left_viewing_distance;

        let mut bot_viewing_distance = 0;
        let bot_view_iter = forest.iter().skip(tree_idx).step_by(line_len).skip(1);
        for &other_tree_height in bot_view_iter {
            bot_viewing_distance += 1;
            if tree_height <= other_tree_height {
                break;
            }
        }
        *scenic_score *= bot_viewing_distance;

        let mut top_viewing_distance = 0;
        let top_view_iter = forest
            .iter()
            .rev()
            .skip(forest.len() - (tree_idx + 1))
            .step_by(line_len)
            .skip(1);
        for &other_tree_height in top_view_iter {
            top_viewing_distance += 1;
            if tree_height <= other_tree_height {
                break;
            }
        }
        *scenic_score *= top_viewing_distance;
    }

    let max_scenic_score = *scenic_score.iter().max().unwrap();

    println!("Part 2: {max_scenic_score}");
}
