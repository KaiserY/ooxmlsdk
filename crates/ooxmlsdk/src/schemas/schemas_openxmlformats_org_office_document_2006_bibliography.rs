//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataSourceValues {
  #[sdk(rename = "ArticleInAPeriodical")]
  #[default]
  ArticleInAPeriodical,
  #[sdk(rename = "Book")]
  Book,
  #[sdk(rename = "BookSection")]
  BookSection,
  #[sdk(rename = "JournalArticle")]
  JournalArticle,
  #[sdk(rename = "ConferenceProceedings")]
  ConferenceProceedings,
  #[sdk(rename = "Report")]
  Report,
  #[sdk(rename = "SoundRecording")]
  SoundRecording,
  #[sdk(rename = "Performance")]
  Performance,
  #[sdk(rename = "Art")]
  Art,
  #[sdk(rename = "DocumentFromInternetSite")]
  DocumentFromInternetSite,
  #[sdk(rename = "InternetSite")]
  InternetSite,
  #[sdk(rename = "Film")]
  Film,
  #[sdk(rename = "Interview")]
  Interview,
  #[sdk(rename = "Patent")]
  Patent,
  #[sdk(rename = "ElectronicSource")]
  ElectronicSource,
  #[sdk(rename = "Case")]
  Case,
  #[sdk(rename = "Misc")]
  Miscellaneous,
}
/// Sources.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_Sources/b:Sources")]
pub struct Sources {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Selected Style
  #[sdk(attr(qname = ":SelectedStyle"))]
  #[sdk(string_length(source = 0u32, min = 0u32, max = 255u32))]
  pub selected_style: Option<crate::simple_type::StringValue>,
  /// Documentation Style Name
  #[sdk(attr(qname = ":StyleName"))]
  #[sdk(string_length(source = 0u32, min = 0u32, max = 255u32))]
  pub style_name: Option<crate::simple_type::StringValue>,
  /// Uniform Resource Identifier
  #[sdk(attr(qname = ":URI"))]
  #[sdk(string_length(source = 0u32, min = 0u32, max = 255u32))]
  pub uri: Option<crate::simple_type::StringValue>,
  /// Source.
  #[sdk(child(qname = "b:CT_SourceType/b:Source"))]
  pub b_source: Vec<Source>,
}
/// Person.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_PersonType/b:Person")]
pub struct Person {
  /// Person's Last, or Family, Name.
  #[sdk(text_child(qname = "b:ST_String255/b:Last"))]
  pub b_last: Vec<crate::simple_type::StringValue>,
  /// Person's First, or Given, Name.
  #[sdk(text_child(qname = "b:ST_String255/b:First"))]
  pub b_first: Vec<crate::simple_type::StringValue>,
  /// Person's Middle, or Other, Name.
  #[sdk(text_child(qname = "b:ST_String255/b:Middle"))]
  pub b_middle: Vec<crate::simple_type::StringValue>,
}
/// Person's Last, or Family, Name.
pub type Last = crate::simple_type::StringValue;
/// Person's First, or Given, Name.
pub type First = crate::simple_type::StringValue;
/// Person's Middle, or Other, Name.
pub type Middle = crate::simple_type::StringValue;
/// Corporate Author.
pub type Corporate = crate::simple_type::StringValue;
/// Abbreviated Case Number.
pub type AbbreviatedCaseNumber = crate::simple_type::StringValue;
/// Album Title.
pub type AlbumTitle = crate::simple_type::StringValue;
/// Book Title.
pub type BookTitle = crate::simple_type::StringValue;
/// Broadcaster.
pub type Broadcaster = crate::simple_type::StringValue;
/// Broadcast Title.
pub type BroadcastTitle = crate::simple_type::StringValue;
/// Case Number.
pub type CaseNumber = crate::simple_type::StringValue;
/// Chapter Number.
pub type ChapterNumber = crate::simple_type::StringValue;
/// City.
pub type City = crate::simple_type::StringValue;
/// Comments.
pub type Comments = crate::simple_type::StringValue;
/// Conference or Proceedings Name.
pub type ConferenceName = crate::simple_type::StringValue;
/// Country or Region.
pub type CountryRegion = crate::simple_type::StringValue;
/// Court.
pub type Court = crate::simple_type::StringValue;
/// Day.
pub type Day = crate::simple_type::StringValue;
/// Day Accessed.
pub type DayAccessed = crate::simple_type::StringValue;
/// Department.
pub type Department = crate::simple_type::StringValue;
/// Distributor.
pub type Distributor = crate::simple_type::StringValue;
/// Editor.
pub type Edition = crate::simple_type::StringValue;
/// GUID.
pub type GuidString = crate::simple_type::StringValue;
/// Institution.
pub type Institution = crate::simple_type::StringValue;
/// Internet Site Title.
pub type InternetSiteTitle = crate::simple_type::StringValue;
/// Issue.
pub type Issue = crate::simple_type::StringValue;
/// Journal Name.
pub type JournalName = crate::simple_type::StringValue;
/// Locale ID.
pub type LcId = crate::simple_type::StringValue;
/// Medium.
pub type Medium = crate::simple_type::StringValue;
/// Month.
pub type Month = crate::simple_type::StringValue;
/// Month Accessed.
pub type MonthAccessed = crate::simple_type::StringValue;
/// Number of Volumes.
pub type NumberVolumes = crate::simple_type::StringValue;
/// Pages.
pub type Pages = crate::simple_type::StringValue;
/// Patent Number.
pub type PatentNumber = crate::simple_type::StringValue;
/// Periodical Title.
pub type PeriodicalTitle = crate::simple_type::StringValue;
/// Production Company.
pub type ProductionCompany = crate::simple_type::StringValue;
/// Publication Title.
pub type PublicationTitle = crate::simple_type::StringValue;
/// Publisher.
pub type Publisher = crate::simple_type::StringValue;
/// Recording Number.
pub type RecordingNumber = crate::simple_type::StringValue;
/// Reference Order.
pub type ReferenceOrder = crate::simple_type::StringValue;
/// Reporter.
pub type Reporter = crate::simple_type::StringValue;
/// Short Title.
pub type ShortTitle = crate::simple_type::StringValue;
/// Standard Number.
pub type StandardNumber = crate::simple_type::StringValue;
/// State or Province.
pub type StateProvince = crate::simple_type::StringValue;
/// Station.
pub type Station = crate::simple_type::StringValue;
/// Tag.
pub type Tag = crate::simple_type::StringValue;
/// Theater.
pub type Theater = crate::simple_type::StringValue;
/// Thesis Type.
pub type ThesisType = crate::simple_type::StringValue;
/// Title.
pub type Title = crate::simple_type::StringValue;
/// Type.
pub type PatentType = crate::simple_type::StringValue;
/// URL.
pub type UrlString = crate::simple_type::StringValue;
/// Version.
pub type Version = crate::simple_type::StringValue;
/// Volume.
pub type Volume = crate::simple_type::StringValue;
/// Year.
pub type Year = crate::simple_type::StringValue;
/// Year Accessed.
pub type YearAccessed = crate::simple_type::StringValue;
/// Name List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameListType/b:NameList")]
pub struct NameList {
  /// Person.
  #[sdk(child(qname = "b:CT_PersonType/b:Person"))]
  pub b_person: Vec<Person>,
}
/// Artist.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Artist")]
pub struct Artist {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Book Author.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:BookAuthor")]
pub struct BookAuthor {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Compiler.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Compiler")]
pub struct Compiler {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Composer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Composer")]
pub struct Composer {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Conductor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Conductor")]
pub struct Conductor {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Counsel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Counsel")]
pub struct Counsel {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Director.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Director")]
pub struct Director {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Editor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Editor")]
pub struct Editor {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Interviewee.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Interviewee")]
pub struct Interviewee {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Interviewer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Interviewer")]
pub struct Interviewer {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Inventor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Inventor")]
pub struct Inventor {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Producer Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:ProducerName")]
pub struct ProducerName {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Translator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Translator")]
pub struct Translator {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Writer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameType/b:Writer")]
pub struct Writer {
  /// Name List
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Author.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameOrCorporateType/b:Author")]
pub struct Author {
  #[sdk(choice(
    qname = "b:CT_NameListType/b:NameList",
    qname = "b:ST_String255/b:Corporate"
  ))]
  pub author_choice: Option<AuthorChoice>,
}
/// Performer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_NameOrCorporateType/b:Performer")]
pub struct Performer {
  #[sdk(choice(
    qname = "b:CT_NameListType/b:NameList",
    qname = "b:ST_String255/b:Corporate"
  ))]
  pub performer_choice: Option<PerformerChoice>,
}
/// Contributors List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_AuthorType/b:Author")]
pub struct AuthorList {
  #[sdk(choice(
    qname = "b:CT_NameType/b:Artist",
    qname = "b:CT_NameOrCorporateType/b:Author",
    qname = "b:CT_NameType/b:BookAuthor",
    qname = "b:CT_NameType/b:Compiler",
    qname = "b:CT_NameType/b:Composer",
    qname = "b:CT_NameType/b:Conductor",
    qname = "b:CT_NameType/b:Counsel",
    qname = "b:CT_NameType/b:Director",
    qname = "b:CT_NameType/b:Editor",
    qname = "b:CT_NameType/b:Interviewee",
    qname = "b:CT_NameType/b:Interviewer",
    qname = "b:CT_NameType/b:Inventor",
    qname = "b:CT_NameOrCorporateType/b:Performer",
    qname = "b:CT_NameType/b:ProducerName",
    qname = "b:CT_NameType/b:Translator",
    qname = "b:CT_NameType/b:Writer"
  ))]
  pub author_list_choice: Vec<AuthorListChoice>,
}
/// Source Type.
pub type SourceType = DataSourceValues;
/// Source.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:CT_SourceType/b:Source")]
pub struct Source {
  #[sdk(choice(
    qname = "b:ST_String255/b:AbbreviatedCaseNumber",
    qname = "b:ST_String255/b:AlbumTitle",
    qname = "b:CT_AuthorType/b:Author",
    qname = "b:ST_String255/b:BookTitle",
    qname = "b:ST_String255/b:Broadcaster",
    qname = "b:ST_String255/b:BroadcastTitle",
    qname = "b:ST_String255/b:CaseNumber",
    qname = "b:ST_String255/b:ChapterNumber",
    qname = "b:ST_String255/b:City",
    qname = "b:ST_String255/b:Comments",
    qname = "b:ST_String255/b:ConferenceName",
    qname = "b:ST_String255/b:CountryRegion",
    qname = "b:ST_String255/b:Court",
    qname = "b:ST_String255/b:Day",
    qname = "b:ST_String255/b:DayAccessed",
    qname = "b:ST_String255/b:Department",
    qname = "b:ST_String255/b:Distributor",
    qname = "b:ST_String255/b:Edition",
    qname = "b:ST_String255/b:Guid",
    qname = "b:ST_String255/b:Institution",
    qname = "b:ST_String255/b:InternetSiteTitle",
    qname = "b:ST_String255/b:Issue",
    qname = "b:ST_String255/b:JournalName",
    qname = "b:ST_String255/b:LCID",
    qname = "b:ST_String255/b:Medium",
    qname = "b:ST_String255/b:Month",
    qname = "b:ST_String255/b:MonthAccessed",
    qname = "b:ST_String255/b:NumberVolumes",
    qname = "b:ST_String255/b:Pages",
    qname = "b:ST_String255/b:PatentNumber",
    qname = "b:ST_String255/b:PeriodicalTitle",
    qname = "b:ST_String255/b:ProductionCompany",
    qname = "b:ST_String255/b:PublicationTitle",
    qname = "b:ST_String255/b:Publisher",
    qname = "b:ST_String255/b:RecordingNumber",
    qname = "b:ST_String255/b:RefOrder",
    qname = "b:ST_String255/b:Reporter",
    qname = "b:ST_SourceType/b:SourceType",
    qname = "b:ST_String255/b:ShortTitle",
    qname = "b:ST_String255/b:StandardNumber",
    qname = "b:ST_String255/b:StateProvince",
    qname = "b:ST_String255/b:Station",
    qname = "b:ST_String255/b:Tag",
    qname = "b:ST_String255/b:Theater",
    qname = "b:ST_String255/b:ThesisType",
    qname = "b:ST_String255/b:Title",
    qname = "b:ST_String255/b:Type",
    qname = "b:ST_String255/b:URL",
    qname = "b:ST_String255/b:Version",
    qname = "b:ST_String255/b:Volume",
    qname = "b:ST_String255/b:Year",
    qname = "b:ST_String255/b:YearAccessed"
  ))]
  pub source_choice: Vec<SourceChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AuthorChoice {
  /// Name List.
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  BNameList(std::boxed::Box<NameList>),
  /// Corporate Author.
  #[sdk(text_child(qname = "b:ST_String255/b:Corporate"))]
  BCorporate(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PerformerChoice {
  /// Name List.
  #[sdk(child(qname = "b:CT_NameListType/b:NameList"))]
  BNameList(std::boxed::Box<NameList>),
  /// Corporate Author.
  #[sdk(text_child(qname = "b:ST_String255/b:Corporate"))]
  BCorporate(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AuthorListChoice {
  /// Artist.
  #[sdk(child(qname = "b:CT_NameType/b:Artist"))]
  BArtist(std::boxed::Box<Artist>),
  /// Author.
  #[sdk(child(qname = "b:CT_NameOrCorporateType/b:Author"))]
  BAuthor(std::boxed::Box<Author>),
  /// Book Author.
  #[sdk(child(qname = "b:CT_NameType/b:BookAuthor"))]
  BBookAuthor(std::boxed::Box<BookAuthor>),
  /// Compiler.
  #[sdk(child(qname = "b:CT_NameType/b:Compiler"))]
  BCompiler(std::boxed::Box<Compiler>),
  /// Composer.
  #[sdk(child(qname = "b:CT_NameType/b:Composer"))]
  BComposer(std::boxed::Box<Composer>),
  /// Conductor.
  #[sdk(child(qname = "b:CT_NameType/b:Conductor"))]
  BConductor(std::boxed::Box<Conductor>),
  /// Counsel.
  #[sdk(child(qname = "b:CT_NameType/b:Counsel"))]
  BCounsel(std::boxed::Box<Counsel>),
  /// Director.
  #[sdk(child(qname = "b:CT_NameType/b:Director"))]
  BDirector(std::boxed::Box<Director>),
  /// Editor.
  #[sdk(child(qname = "b:CT_NameType/b:Editor"))]
  BEditor(std::boxed::Box<Editor>),
  /// Interviewee.
  #[sdk(child(qname = "b:CT_NameType/b:Interviewee"))]
  BInterviewee(std::boxed::Box<Interviewee>),
  /// Interviewer.
  #[sdk(child(qname = "b:CT_NameType/b:Interviewer"))]
  BInterviewer(std::boxed::Box<Interviewer>),
  /// Inventor.
  #[sdk(child(qname = "b:CT_NameType/b:Inventor"))]
  BInventor(std::boxed::Box<Inventor>),
  /// Performer.
  #[sdk(child(qname = "b:CT_NameOrCorporateType/b:Performer"))]
  BPerformer(std::boxed::Box<Performer>),
  /// Producer Name.
  #[sdk(child(qname = "b:CT_NameType/b:ProducerName"))]
  BProducerName(std::boxed::Box<ProducerName>),
  /// Translator.
  #[sdk(child(qname = "b:CT_NameType/b:Translator"))]
  BTranslator(std::boxed::Box<Translator>),
  /// Writer.
  #[sdk(child(qname = "b:CT_NameType/b:Writer"))]
  BWriter(std::boxed::Box<Writer>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SourceChoice {
  /// Abbreviated Case Number.
  #[sdk(text_child(qname = "b:ST_String255/b:AbbreviatedCaseNumber"))]
  BAbbreviatedCaseNumber(crate::simple_type::StringValue),
  /// Album Title.
  #[sdk(text_child(qname = "b:ST_String255/b:AlbumTitle"))]
  BAlbumTitle(crate::simple_type::StringValue),
  /// Contributors List.
  #[sdk(child(qname = "b:CT_AuthorType/b:Author"))]
  BAuthor(std::boxed::Box<AuthorList>),
  /// Book Title.
  #[sdk(text_child(qname = "b:ST_String255/b:BookTitle"))]
  BBookTitle(crate::simple_type::StringValue),
  /// Broadcaster.
  #[sdk(text_child(qname = "b:ST_String255/b:Broadcaster"))]
  BBroadcaster(crate::simple_type::StringValue),
  /// Broadcast Title.
  #[sdk(text_child(qname = "b:ST_String255/b:BroadcastTitle"))]
  BBroadcastTitle(crate::simple_type::StringValue),
  /// Case Number.
  #[sdk(text_child(qname = "b:ST_String255/b:CaseNumber"))]
  BCaseNumber(crate::simple_type::StringValue),
  /// Chapter Number.
  #[sdk(text_child(qname = "b:ST_String255/b:ChapterNumber"))]
  BChapterNumber(crate::simple_type::StringValue),
  /// City.
  #[sdk(text_child(qname = "b:ST_String255/b:City"))]
  BCity(crate::simple_type::StringValue),
  /// Comments.
  #[sdk(text_child(qname = "b:ST_String255/b:Comments"))]
  BComments(crate::simple_type::StringValue),
  /// Conference or Proceedings Name.
  #[sdk(text_child(qname = "b:ST_String255/b:ConferenceName"))]
  BConferenceName(crate::simple_type::StringValue),
  /// Country or Region.
  #[sdk(text_child(qname = "b:ST_String255/b:CountryRegion"))]
  BCountryRegion(crate::simple_type::StringValue),
  /// Court.
  #[sdk(text_child(qname = "b:ST_String255/b:Court"))]
  BCourt(crate::simple_type::StringValue),
  /// Day.
  #[sdk(text_child(qname = "b:ST_String255/b:Day"))]
  BDay(crate::simple_type::StringValue),
  /// Day Accessed.
  #[sdk(text_child(qname = "b:ST_String255/b:DayAccessed"))]
  BDayAccessed(crate::simple_type::StringValue),
  /// Department.
  #[sdk(text_child(qname = "b:ST_String255/b:Department"))]
  BDepartment(crate::simple_type::StringValue),
  /// Distributor.
  #[sdk(text_child(qname = "b:ST_String255/b:Distributor"))]
  BDistributor(crate::simple_type::StringValue),
  /// Editor.
  #[sdk(text_child(qname = "b:ST_String255/b:Edition"))]
  BEdition(crate::simple_type::StringValue),
  /// GUID.
  #[sdk(text_child(qname = "b:ST_String255/b:Guid"))]
  BGuid(crate::simple_type::StringValue),
  /// Institution.
  #[sdk(text_child(qname = "b:ST_String255/b:Institution"))]
  BInstitution(crate::simple_type::StringValue),
  /// Internet Site Title.
  #[sdk(text_child(qname = "b:ST_String255/b:InternetSiteTitle"))]
  BInternetSiteTitle(crate::simple_type::StringValue),
  /// Issue.
  #[sdk(text_child(qname = "b:ST_String255/b:Issue"))]
  BIssue(crate::simple_type::StringValue),
  /// Journal Name.
  #[sdk(text_child(qname = "b:ST_String255/b:JournalName"))]
  BJournalName(crate::simple_type::StringValue),
  /// Locale ID.
  #[sdk(text_child(qname = "b:ST_String255/b:LCID"))]
  BLcid(crate::simple_type::StringValue),
  /// Medium.
  #[sdk(text_child(qname = "b:ST_String255/b:Medium"))]
  BMedium(crate::simple_type::StringValue),
  /// Month.
  #[sdk(text_child(qname = "b:ST_String255/b:Month"))]
  BMonth(crate::simple_type::StringValue),
  /// Month Accessed.
  #[sdk(text_child(qname = "b:ST_String255/b:MonthAccessed"))]
  BMonthAccessed(crate::simple_type::StringValue),
  /// Number of Volumes.
  #[sdk(text_child(qname = "b:ST_String255/b:NumberVolumes"))]
  BNumberVolumes(crate::simple_type::StringValue),
  /// Pages.
  #[sdk(text_child(qname = "b:ST_String255/b:Pages"))]
  BPages(crate::simple_type::StringValue),
  /// Patent Number.
  #[sdk(text_child(qname = "b:ST_String255/b:PatentNumber"))]
  BPatentNumber(crate::simple_type::StringValue),
  /// Periodical Title.
  #[sdk(text_child(qname = "b:ST_String255/b:PeriodicalTitle"))]
  BPeriodicalTitle(crate::simple_type::StringValue),
  /// Production Company.
  #[sdk(text_child(qname = "b:ST_String255/b:ProductionCompany"))]
  BProductionCompany(crate::simple_type::StringValue),
  /// Publication Title.
  #[sdk(text_child(qname = "b:ST_String255/b:PublicationTitle"))]
  BPublicationTitle(crate::simple_type::StringValue),
  /// Publisher.
  #[sdk(text_child(qname = "b:ST_String255/b:Publisher"))]
  BPublisher(crate::simple_type::StringValue),
  /// Recording Number.
  #[sdk(text_child(qname = "b:ST_String255/b:RecordingNumber"))]
  BRecordingNumber(crate::simple_type::StringValue),
  /// Reference Order.
  #[sdk(text_child(qname = "b:ST_String255/b:RefOrder"))]
  BRefOrder(crate::simple_type::StringValue),
  /// Reporter.
  #[sdk(text_child(qname = "b:ST_String255/b:Reporter"))]
  BReporter(crate::simple_type::StringValue),
  /// Source Type.
  #[sdk(text_child(qname = "b:ST_SourceType/b:SourceType"))]
  BSourceType(DataSourceValues),
  /// Short Title.
  #[sdk(text_child(qname = "b:ST_String255/b:ShortTitle"))]
  BShortTitle(crate::simple_type::StringValue),
  /// Standard Number.
  #[sdk(text_child(qname = "b:ST_String255/b:StandardNumber"))]
  BStandardNumber(crate::simple_type::StringValue),
  /// State or Province.
  #[sdk(text_child(qname = "b:ST_String255/b:StateProvince"))]
  BStateProvince(crate::simple_type::StringValue),
  /// Station.
  #[sdk(text_child(qname = "b:ST_String255/b:Station"))]
  BStation(crate::simple_type::StringValue),
  /// Tag.
  #[sdk(text_child(qname = "b:ST_String255/b:Tag"))]
  BTag(crate::simple_type::StringValue),
  /// Theater.
  #[sdk(text_child(qname = "b:ST_String255/b:Theater"))]
  BTheater(crate::simple_type::StringValue),
  /// Thesis Type.
  #[sdk(text_child(qname = "b:ST_String255/b:ThesisType"))]
  BThesisType(crate::simple_type::StringValue),
  /// Title.
  #[sdk(text_child(qname = "b:ST_String255/b:Title"))]
  BTitle(crate::simple_type::StringValue),
  /// Type.
  #[sdk(text_child(qname = "b:ST_String255/b:Type"))]
  BType(crate::simple_type::StringValue),
  /// URL.
  #[sdk(text_child(qname = "b:ST_String255/b:URL"))]
  BUrl(crate::simple_type::StringValue),
  /// Version.
  #[sdk(text_child(qname = "b:ST_String255/b:Version"))]
  BVersion(crate::simple_type::StringValue),
  /// Volume.
  #[sdk(text_child(qname = "b:ST_String255/b:Volume"))]
  BVolume(crate::simple_type::StringValue),
  /// Year.
  #[sdk(text_child(qname = "b:ST_String255/b:Year"))]
  BYear(crate::simple_type::StringValue),
  /// Year Accessed.
  #[sdk(text_child(qname = "b:ST_String255/b:YearAccessed"))]
  BYearAccessed(crate::simple_type::StringValue),
}
