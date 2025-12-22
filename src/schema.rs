use duckdb::core::LogicalTypeId;
use warc::WarcHeader;

#[derive(Debug)]
pub struct Field {
    pub header: WarcHeader,
    pub field_type: LogicalTypeId,
}

pub static WARC_FIELDS: &[Field] = &[
    Field { header: WarcHeader::RecordID, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::ContentLength, field_type: LogicalTypeId::Integer },
    Field { header: WarcHeader::Date, field_type: LogicalTypeId::Timestamp },
    Field { header: WarcHeader::WarcType, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::ContentType, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::ConcurrentTo, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::BlockDigest, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::PayloadDigest, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::IPAddress, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::RefersTo, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::TargetURI, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::Truncated, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::WarcInfoID, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::Filename, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::Profile, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::IdentifiedPayloadType, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::SegmentNumber, field_type: LogicalTypeId::Integer },
    Field { header: WarcHeader::SegmentOriginID, field_type: LogicalTypeId::Varchar },
    Field { header: WarcHeader::SegmentTotalLength, field_type: LogicalTypeId::Integer },
];
