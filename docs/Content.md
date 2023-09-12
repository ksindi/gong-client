# Content

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_id** | Option<**String**> | A unique identifier for the content in the partner's system | [optional]
**content_title** | **String** | The title of the content | 
**content_label** | Option<**Vec<String>**> | A list of tags defined for the content | [optional]
**content_url** | Option<**String**> | The URL for the content the person looked at | [optional]
**content_additional_info_url** | Option<**String**> | The URL for additional details provided by the website or external app, such as analysis of the content viewed | [optional]
**numeric_content_details** | Option<[**crate::models::NumericContentDetails**](NumericContentDetails.md)> |  | [optional]
**step_content_details** | Option<[**crate::models::StepContentDetails**](StepContentDetails.md)> |  | [optional]
**search_object_details** | Option<[**crate::models::SearchObjectDetails**](SearchObjectDetails.md)> |  | [optional]
**content_custom_fields** | Option<[**Vec<crate::models::CustomField>**](CustomField.md)> | Include additional custom data about the content | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


