# SpeakersTimeline

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**precise** | Option<**bool**> | Indicates whether the provided speech segments match the media precisely or need further refinement based on the media. \"Precisely\" here means that segments do not deviate from the actual speech in the media by more than 100ms. | [optional]
**speech_segments** | Option<[**Vec<crate::models::SpeechSegment>**](SpeechSegment.md)> | The audio recording speech segments (who spoke when).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


