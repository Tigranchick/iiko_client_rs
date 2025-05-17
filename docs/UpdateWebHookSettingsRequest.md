# UpdateWebHookSettingsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization UOC Id.                Can be obtained by `/api/1/organizations` operation. | 
**web_hooks_uri** | **String** | Webhook handler url. | 
**auth_token** | Option<**String**> | Authorization token to pass to the webhook handler. | [optional]
**web_hooks_filter** | Option<[**models::WebHooksFilter**](WebHooksFilter.md)> | Webhooks filter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


