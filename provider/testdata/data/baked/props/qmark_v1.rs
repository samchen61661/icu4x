// @generated
type DataStruct =
    <::icu_properties::provider::QuotationMarkV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    34u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 40u8, 0u8, 0u8,
                    0u8, 171u8, 0u8, 0u8, 0u8, 172u8, 0u8, 0u8, 0u8, 187u8, 0u8, 0u8, 0u8, 188u8,
                    0u8, 0u8, 0u8, 24u8, 32u8, 0u8, 0u8, 32u8, 32u8, 0u8, 0u8, 57u8, 32u8, 0u8,
                    0u8, 59u8, 32u8, 0u8, 0u8, 66u8, 46u8, 0u8, 0u8, 67u8, 46u8, 0u8, 0u8, 12u8,
                    48u8, 0u8, 0u8, 16u8, 48u8, 0u8, 0u8, 29u8, 48u8, 0u8, 0u8, 32u8, 48u8, 0u8,
                    0u8, 65u8, 254u8, 0u8, 0u8, 69u8, 254u8, 0u8, 0u8, 2u8, 255u8, 0u8, 0u8, 3u8,
                    255u8, 0u8, 0u8, 7u8, 255u8, 0u8, 0u8, 8u8, 255u8, 0u8, 0u8, 98u8, 255u8, 0u8,
                    0u8, 100u8, 255u8, 0u8, 0u8,
                ])
            },
            30usize,
        )
    });
