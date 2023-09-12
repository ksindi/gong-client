# MultipleUsersRequestWithCreationDates

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_from_date_time** | Option<**String**> | An optional user creation time lower limit, if supplied the API will return only the users created at or after this time. The filed is in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | [optional]
**created_to_date_time** | Option<**String**> | An optional user creation time upper limit, if supplied the API will return only the users created before this time. The filed is in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | [optional]
**include_avatars** | Option<**bool**> | Avatars are synthetic users representing Gong employees (CSMs and support providers) when they access your instance. References to avatars' IDs may be found in the outputs of other API endpoints. This parameter is optional, if not provided avatars will not be included in the results. | [optional]
**user_ids** | Option<**Vec<String>**> | Set of Gong's unique numeric identifiers for the users (up to 20 digits). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


