# Monologue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**speaker_id** | Option<**String**> | Unique ID of the speaker. Cross-reference this with the 'speakerId' field of objects in the 'parties' array returned from endpoint '/v2/calls/extensive' to identify the participant.  | [optional]
**topic** | Option<**String**> | The name of the topic. | [optional]
**sentences** | Option<[**Vec<crate::models::Sentence>**](Sentence.md)> | A list of sentences spoken in the monologue. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


