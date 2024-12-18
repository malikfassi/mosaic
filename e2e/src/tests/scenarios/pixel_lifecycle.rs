use crate::tests::test_utils::*;
use cosmwasm_std::testing::mock_dependencies;

#[test]
fn test_pixel_creation() {
    let (mut deps, env, info) = setup_test_contract();
    
    // Create a pixel
    create_test_pixel(&mut deps, &env, &info, 0, 0, "#FF0000".to_string());
    
    // Query the pixel
    let color = query_pixel(&deps.as_ref(), 0, 0).unwrap();
    assert_eq!(color, "#FF0000");
}

#[test]
fn test_pixel_update() {
    let (mut deps, env, info) = setup_test_contract();
    
    // Create a pixel
    create_test_pixel(&mut deps, &env, &info, 0, 0, "#FF0000".to_string());
    
    // Update the pixel
    create_test_pixel(&mut deps, &env, &info, 0, 0, "#00FF00".to_string());
    
    // Query the pixel
    let color = query_pixel(&deps.as_ref(), 0, 0).unwrap();
    assert_eq!(color, "#00FF00");
}

#[test]
fn test_invalid_pixel_coordinates() {
    let (mut deps, env, info) = setup_test_contract();
    
    // Try to create a pixel with invalid coordinates
    // TODO: Add validation test when contract is ready
} 