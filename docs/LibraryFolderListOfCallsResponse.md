# LibraryFolderListOfCallsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | A Gong request reference Id, generated for this request. Can be used for troubleshooting purposes. | [optional]
**id** | Option<**String**> | Gong's unique numeric identifier for a library's folder (up to 20 digits). | [optional]
**name** | Option<**String**> | Display name of the folder. | [optional]
**created_by** | Option<**String**> | Gong's unique numeric identifier for the user who added the folder. | [optional]
**updated** | Option<**String**> | Folder's last update time in the ISO-8601 format (e.g., '2020-03-14T05:30:00-08:00'). | [optional]
**calls** | Option<[**Vec<crate::models::LibraryCall>**](LibraryCall.md)> | A list, in which each item represents one library call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


