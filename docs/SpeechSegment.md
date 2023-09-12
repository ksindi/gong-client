# SpeechSegment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_time** | **i32** | The start time of the segment in milliseconds from the beginning of the call. | 
**to_time** | **i32** | The end time of the segment in milliseconds from the beginning of the call. | 
**party_ids** | **Vec<String>** | The speaking parties in the segment, each must have a correlating partyId within 'parties'. It is allowed to provide overlapping segments, i.e. you can either specify multiple speakers in a segment or send several overlapping segments each marked with one speaker. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


