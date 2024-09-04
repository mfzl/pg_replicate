use reqwest::StatusCode;

use crate::test_app::{
    spawn_app, CreateImageRequest, CreateImageResponse, ImageResponse, TestApp, UpdateImageRequest,
};

pub async fn create_default_image(app: &TestApp) -> i64 {
    create_image_with_name(app, "some/image".to_string(), true).await
}

pub async fn create_image_with_name(app: &TestApp, name: String, is_default: bool) -> i64 {
    let image = CreateImageRequest { name, is_default };
    let response = app.create_image(&image).await;
    let response: CreateImageResponse = response
        .json()
        .await
        .expect("failed to deserialize response");
    response.id
}

#[tokio::test]
async fn image_can_be_created() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let image = CreateImageRequest {
        name: "some/image".to_string(),
        is_default: true,
    };
    let response = app.create_image(&image).await;

    // Assert
    assert!(response.status().is_success());
    let response: CreateImageResponse = response
        .json()
        .await
        .expect("failed to deserialize response");
    assert_eq!(response.id, 1);
}

#[tokio::test]
async fn an_existing_image_can_be_read() {
    // Arrange
    let app = spawn_app().await;

    let name = "some/image".to_string();
    let is_default = true;
    let image = CreateImageRequest {
        name: name.clone(),
        is_default,
    };
    let response = app.create_image(&image).await;
    let response: CreateImageResponse = response
        .json()
        .await
        .expect("failed to deserialize response");
    let image_id = response.id;

    // Act
    let response = app.read_image(image_id).await;

    // Assert
    assert!(response.status().is_success());
    let response: ImageResponse = response
        .json()
        .await
        .expect("failed to deserialize response");
    assert_eq!(response.id, image_id);
    assert_eq!(response.name, name);
    assert_eq!(response.is_default, is_default);
}

#[tokio::test]
async fn an_non_existing_image_cant_be_read() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.read_image(42).await;

    // Assert
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn an_existing_image_can_be_updated() {
    // Arrange
    let app = spawn_app().await;

    let name = "some/image".to_string();
    let is_default = true;
    let image = CreateImageRequest {
        name: name.clone(),
        is_default,
    };
    let response = app.create_image(&image).await;
    let response: CreateImageResponse = response
        .json()
        .await
        .expect("failed to deserialize response");
    let image_id = response.id;

    // Act
    let name = "some/image".to_string();
    let is_default = true;
    let updated_image = UpdateImageRequest {
        name: name.clone(),
        is_default,
    };
    let response = app.update_image(image_id, &updated_image).await;

    // Assert
    assert!(response.status().is_success());
    let response = app.read_image(image_id).await;
    let response: ImageResponse = response
        .json()
        .await
        .expect("failed to deserialize response");
    assert_eq!(response.id, image_id);
    assert_eq!(response.name, name);
    assert_eq!(response.is_default, is_default);
}

#[tokio::test]
async fn an_non_existing_source_cant_be_updated() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let name = "some/image".to_string();
    let is_default = true;
    let updated_image = UpdateImageRequest {
        name: name.clone(),
        is_default,
    };
    let response = app.update_image(42, &updated_image).await;

    // Assert
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn an_existing_image_can_be_deleted() {
    // Arrange
    let app = spawn_app().await;

    let name = "some/image".to_string();
    let is_default = true;
    let image = CreateImageRequest {
        name: name.clone(),
        is_default,
    };
    let response = app.create_image(&image).await;
    let response: CreateImageResponse = response
        .json()
        .await
        .expect("failed to deserialize response");
    let image_id = response.id;

    // Act
    let response = app.delete_image(image_id).await;

    // Assert
    assert!(response.status().is_success());
    let response = app.read_image(image_id).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn an_non_existing_image_cant_be_deleted() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.delete_image(42).await;

    // Assert
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn all_images_can_be_read() {
    // Arrange
    let app = spawn_app().await;
    let image1_id = create_image_with_name(&app, "some/image".to_string(), true).await;
    let image2_id = create_image_with_name(&app, "other/image".to_string(), false).await;

    // Act
    let response = app.read_all_images().await;

    // Assert
    assert!(response.status().is_success());
    let response: Vec<ImageResponse> = response
        .json()
        .await
        .expect("failed to deserialize response");
    for image in response {
        if image.id == image1_id {
            assert_eq!(image.name, "some/image");
            assert!(image.is_default);
        } else if image.id == image2_id {
            assert_eq!(image.name, "other/image");
            assert!(!image.is_default);
        }
    }
}