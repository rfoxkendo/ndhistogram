mod test_axes;
mod test_binrange;
mod test_category_axis;
mod test_category_binrange;
mod test_categorynoflow_axis;
mod test_hashhistogram;
mod test_high_dimensional_axes;
mod test_ndhistogram_1d;
mod test_ndhistogram_1d_category;
mod test_ndhistogram_2d;
mod test_ndhistogram_binary_ops;
mod test_ndhistogram_display;
mod test_ndhistogram_macro;
mod test_noflow_axis;

#[cfg(feature = "rayon")]
mod test_parallel_iterators;
#[cfg(feature = "serde")]
mod test_serialization;

mod test_trait_object_safety;
mod test_uniform_axis;
mod test_uniform_axis_integer;
mod test_uniformcyclic_axis;
mod test_value_mean;
mod test_value_sum;
mod test_value_weightedmean;
mod test_value_weightedsum;
mod test_variable_axis;
mod test_variablecyclic_axis;
mod test_variablenoflow_axis;
mod test_versions;
