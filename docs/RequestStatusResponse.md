# RequestStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | A Gong request reference Id, generated for this request. Can be used for troubleshooting purposes. | [optional]
**status** | Option<**String**> | Request status | [optional]
**errors** | Option<[**Vec<crate::models::LineErrorResponse>**](LineErrorResponse.md)> |  | [optional]
**total_error_count** | Option<**i32**> | Number of objects that failed parsing | [optional]
**total_success_count** | Option<**i32**> | Number of valid objects | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


