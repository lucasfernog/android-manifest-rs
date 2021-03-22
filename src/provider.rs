use super::resources::{DrawableResource, Resource, StringResourceOrString};
use super::path_permission::PathPermission;
use super::intent_filter::IntentFilter;
use super::meta_data::MetaData;
use super::grant_uri_permission::GrantUriPermission;
use serde::{Deserialize, Serialize};

/// Declares a content provider component. A content provider is a subclass of `ContentProvider` that supplies structured access to data managed by the application.
/// All content providers in your application must be defined in a `<provider>` element in the manifest file; otherwise, the system is unaware of them and doesn't run them.
/// You only declare content providers that are part of your application. Content providers in other applications that you use in your application should not be declared.
/// The Android system stores references to content providers according to an authority string, part of the provider's content URI.
/// For example, suppose you want to access a content provider that stores information about health care professionals.
/// To do this, you call the method `ContentResolver.query()`, which among other arguments takes a URI that identifies the provider:
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "provider")]
pub struct Provider {
    /// A list of one or more URI authorities that identify data offered by the content provider. Multiple authorities are listed by separating their names with a semicolon.
    /// To avoid conflicts, authority names should use a Java-style naming convention (such as com.example.provider.cartoonprovider).
    /// Typically, it's the name of the ContentProvider subclass that implements the provider
    /// There is no default. At least one authority must be specified.
    #[serde(rename = "android:authorities", with = "authorities")]
    pub authorities: Vec<String>,
    /// Whether or not the service can be instantiated by the system — `"true"` if it can be, and `"false"` if not. The default value is `"true"`.
    /// The `<application>` element has its own enabled attribute that applies to all application components, including services.
    /// The `<application>` and <service> attributes must both be `"true"` (as they both are by default) for the service to be enabled. If either is `"false"`, the service is disabled; it cannot be instantiated.
    #[serde(rename = "android:enabled")]
    pub enabled: Option<bool>,
    /// Whether or not the service is direct-boot aware; that is, whether or not it can run before the user unlocks the device.
    /// Note: During Direct Boot, a service in your application can only access the data that is stored in device protected storage.
    /// The default value is `"false"`.
    #[serde(rename = "android:directBootAware")]
    pub direct_boot_aware: Option<bool>,
    /// Whether the content provider is available for other applications to use:
    /// `true:` The provider is available to other applications. Any application can use the provider's content URI to access it, subject to the permissions specified for the provider.
    /// `false:` The provider is not available to other applications. Set android:exported="false" to limit access to the provider to your applications.
    /// Only applications that have the same user ID (UID) as the provider, or applications that have been temporarily granted access to the provider through the android:grantUriPermissions element, have access to it.
    /// Because this attribute was introduced in API level 17, all devices running API level 16 and lower behave as though this attribute is set `"true"`.
    /// If you set `android:targetSdkVersion` to 17 or higher, then the default value is `"false"` for devices running API level 17 and higher.
    /// You can set `android:exported="false"` and still limit access to your provider by setting permissions with the `permission` attribute.
    #[serde(rename = "android:exported")]
    pub exported: Option<bool>,
    /// Whether or not those who ordinarily would not have permission to access the content provider's data can be granted permission to do so, temporarily overcoming the restriction imposed by the `readPermission`,
    /// `writePermission`, `permission`, and `exported` attributes — `"true"` if permission can be granted, and `"false"` if not. If `"true"`, permission can be granted to any of the content provider's data. If `"false"`,
    /// permission can be granted only to the data subsets listed in `<grant-uri-permission>` subelements, if any.
    /// The default value is `"false"`.
    /// Granting permission is a way of giving an application component one-time access to data protected by a permission. For example, when an e-mail message contains an attachment, the mail application may call
    /// upon the appropriate viewer to open it, even though the viewer doesn't have general permission to look at all the content provider's data.
    /// In such cases, permission is granted by FLAG_GRANT_READ_URI_PERMISSION and FLAG_GRANT_WRITE_URI_PERMISSION flags in the Intent object that activates the component. For example, the
    /// mail application might put `FLAG_GRANT_READ_URI_PERMISSION` in the Intent passed to `Context.startActivity()`. The permission is specific to the URI in the Intent.
    /// If you enable this feature, either by setting this attribute to `"true"` or by defining `<grant-uri-permission>` subelements, you must call `Context.revokeUriPermission()` when a covered URI is deleted from the provider.
    /// See also the `<grant-uri-permission>` element.
    #[serde(rename = "android:grantUriPermissions")]
    pub grant_uri_permissions: Option<bool>,
    /// An icon representing the content provider. This attribute must be set as a reference to a drawable resource containing the image definition.
    /// If it is not set, the icon specified for the application as a whole is used instead (see the `<application>` element's `icon` attribute).
    #[serde(rename = "android:icon")]
    pub icon: Option<Resource<DrawableResource>>,
    /// The order in which the content provider should be instantiated, relative to other content providers hosted by the same process.
    /// When there are dependencies among content providers, setting this attribute for each of them ensures that they are created in the order required by those dependencies.
    /// The value is a simple integer, with higher numbers being initialized first.
    #[serde(rename = "android:initOrder")]
    pub init_order: Option<i32>,
    /// A user-readable label for the content provided. If this attribute is not set, the label set for the application as a whole is used instead (see the `<application>` element's `label` attribute).
    /// The label should be set as a reference to a string resource, so that it can be localized like other strings in the user interface.
    /// However, as a convenience while you're developing the application, it can also be set as a raw string.
    #[serde(rename = "android:label")]
    pub label: Option<StringResourceOrString>,
    /// If the app runs in multiple processes, this attribute determines whether multiple instances of the content provider are created.
    /// If true, each of the app's processes has its own content provider object. If false, the app's processes share only one content provider object. The default value is false.
    /// Setting this flag to true may improve performance by reducing the overhead of interprocess communication, but it also increases the memory footprint of each process.
    #[serde(rename = "android:multiprocess")]
    pub multiprocess: Option<bool>,
    /// The name of the class that implements the content provider, a subclass of `ContentProvider`. This should be a fully qualified class name (such as, `"com.example.project.TransportationProvider"`).
    /// However, as a shorthand, if the first character of the name is a period, it is appended to the package name specified in the `<manifest>` element.
    /// There is no default. The name must be specified.
    #[serde(rename = "android:name")]
    pub name: String,
    /// The name of a permission that clients must have to read or write the content provider's data. This attribute is a convenient way of setting a single permission for both reading and writing. However, the `readPermission,`
    /// `writePermission`, and `grantUriPermissions` attributes take precedence over this one. If the `readPermission` attribute is also set, it controls access for querying the content provider. And if the
    /// `writePermission` attribute is set, it controls access for modifying the provider's data.
    /// For more information on permissions, see the `Permissions` section in the introduction and a separate document, `Security and Permissions`.
    #[serde(rename = "android:permission")]
    pub permission: Option<String>,
    /// The name of the process in which the content provider should run. Normally, all components of an application run in the default process created for the application. It has the same name as the application package.
    /// The `<application>` element's `process` attribute can set a different default for all components. But each
    /// component can override the default with its own `process` attribute, allowing you to spread your application across multiple processes.
    /// If the name assigned to this attribute begins with a colon (':'), a new process, private to the application, is created when it's needed and the activity runs in that process. If the process name begins with a lowercase
    /// character, the activity will run in a global process of that name, provided that it has permission to do so. This allows components in different applications to share a process, reducing resource usage.
    #[serde(rename = "android:process")]
    pub process: Option<String>,
    /// A permission that clients must have to query the content provider.
    /// If the provider sets `android:grantUriPermissions` to true, or if a given client satisfies the conditions of a `<grant-uri-permission>` subelement, the client can gain temporary read access to the content provider's data.
    /// See also the `permission` and `writePermission` attributes.
    #[serde(rename = "android:readPermission")]
    pub read_permission: Option<String>,
    /// Whether or not the data under the content provider's control is to be synchronized with data on a server — `"true"` if it is to be synchronized, and `"false"` if not.
    #[serde(rename = "android:syncable")]
    pub syncable: Option<bool>,
    /// A permission that clients must have to make changes to the data controlled by the content provider.
    /// If the provider sets `android:grantUriPermission`s to true, or if a given client satisfies the conditions of a `<grant-uri-permission>` subelement
    /// the client can gain temporary write access to modify the content provider's data.
    /// See also the `permission` and `readPermission` attributes.
    #[serde(rename = "android:writePermission")]
    pub write_permission: Option<String>,
 
    pub path_permission: Option<PathPermission>,

    pub grant_uri_permission: Option<GrantUriPermission>,

    pub intent_filter: Option<IntentFilter>,

    pub meta_data: Option<MetaData>,

}

mod authorities {
    use serde::{
        de::{self, Visitor},
        ser::Error,
        Deserializer, Serializer,
    };
    use std::fmt;

    pub fn serialize<S>(authorities: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if authorities.is_empty() {
            return Err(S::Error::custom("there is no default `android::authorities`. at least one authority must be specified"));
        };
        serializer.serialize_str(&authorities.join(";"))
    }

    struct AuthoritiesVisitor;

    impl<'de> Visitor<'de> for AuthoritiesVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(&format!(
                "an authorities list in format 'authority1' or 'authority1;authority2;authority3'"
            ))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.is_empty() {
                return Err(E::custom(
                    "there is no default `android::authorities`. at least one authority must be specified"));
            };
            let authorities: Vec<String> = v
                .replace(" ", "")
                .split(';')
                .map(|s| s.to_owned())
                .collect();
            Ok(authorities)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(AuthoritiesVisitor)
    }
}