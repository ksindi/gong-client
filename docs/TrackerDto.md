# TrackerDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique ID of the Tracker. | [optional]
**name** | Option<**String**> | The name of the Tracker (e.g., Stores). | [optional]
**count** | Option<**i32**> | The number of times words in this Tracker occurred in the call. | [optional]
**r#type** | Option<**String**> | The type of the tracker - Keyword or Smart. | [optional]
**occurrences** | Option<[**Vec<crate::models::Occurrences>**](occurrences.md)> | Details when the smart or keyword tracker term came up in the call. The list does not specify each phrase defined in the tracker, rather it lists when any term relevant to the tracker was mentioned. Empty when keyword trackers are set to display each term in the tracker separately. | [optional]
**phrases** | Option<[**Vec<crate::models::TrackerPhrases>**](trackerPhrases.md)> | Details when each specific phrase defined in the keyword tracker came up in the call. Smart trackers are not listed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


