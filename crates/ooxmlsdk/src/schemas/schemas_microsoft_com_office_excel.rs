//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ClipboardFormatValues {
  #[sdk(rename = "PictOld")]
  #[default]
  PictureOld,
  #[sdk(rename = "Pict")]
  Picture,
  #[sdk(rename = "Bitmap")]
  Bitmap,
  #[sdk(rename = "PictPrint")]
  PicturePrint,
  #[sdk(rename = "PictScreen")]
  PictureScreen,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ObjectValues {
  #[sdk(rename = "Button")]
  #[default]
  Button,
  #[sdk(rename = "Checkbox")]
  Checkbox,
  #[sdk(rename = "Dialog")]
  Dialog,
  #[sdk(rename = "Drop")]
  Drop,
  #[sdk(rename = "Edit")]
  Edit,
  #[sdk(rename = "GBox")]
  GroupBox,
  #[sdk(rename = "Label")]
  Label,
  #[sdk(rename = "LineA")]
  AuditingLine,
  #[sdk(rename = "List")]
  List,
  #[sdk(rename = "Movie")]
  Movie,
  #[sdk(rename = "Note")]
  Note,
  #[sdk(rename = "Pict")]
  Picture,
  #[sdk(rename = "Radio")]
  Radio,
  #[sdk(rename = "RectA")]
  AuditingRectangle,
  #[sdk(rename = "Scroll")]
  Scroll,
  #[sdk(rename = "Spin")]
  Spin,
  #[sdk(rename = "Shape")]
  Shape,
  #[sdk(rename = "Group")]
  Group,
  #[sdk(rename = "Rect")]
  Rectangle,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BooleanEntryWithBlankValues {
  #[sdk(rename = "True")]
  #[default]
  True,
  #[sdk(rename = "t")]
  T,
  #[sdk(rename = "False")]
  False,
  #[sdk(rename = "f")]
  F,
  Empty,
}
/// Attached Object Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xvml:CT_ClientData/xvml:ClientData")]
pub struct ClientData {
  /// Object type
  #[sdk(attr(qname = ":ObjectType"))]
  pub object_type: ObjectValues,
  #[sdk(choice(
    qname = "xvml:ST_TrueFalseBlank/xvml:MoveWithCells",
    qname = "xvml:ST_TrueFalseBlank/xvml:SizeWithCells",
    qname = "xsd:string/xvml:Anchor",
    qname = "xvml:ST_TrueFalseBlank/xvml:Locked",
    qname = "xvml:ST_TrueFalseBlank/xvml:DefaultSize",
    qname = "xvml:ST_TrueFalseBlank/xvml:PrintObject",
    qname = "xvml:ST_TrueFalseBlank/xvml:Disabled",
    qname = "xvml:ST_TrueFalseBlank/xvml:AutoFill",
    qname = "xvml:ST_TrueFalseBlank/xvml:AutoLine",
    qname = "xvml:ST_TrueFalseBlank/xvml:AutoPict",
    qname = "xvml:ST_Macro/xvml:FmlaMacro",
    qname = "xsd:string/xvml:TextHAlign",
    qname = "xsd:string/xvml:TextVAlign",
    qname = "xvml:ST_TrueFalseBlank/xvml:LockText",
    qname = "xvml:ST_TrueFalseBlank/xvml:JustLastX",
    qname = "xvml:ST_TrueFalseBlank/xvml:SecretEdit",
    qname = "xvml:ST_TrueFalseBlank/xvml:Default",
    qname = "xvml:ST_TrueFalseBlank/xvml:Help",
    qname = "xvml:ST_TrueFalseBlank/xvml:Cancel",
    qname = "xvml:ST_TrueFalseBlank/xvml:Dismiss",
    qname = "xsd:unsignedByte/xvml:Accel",
    qname = "xsd:unsignedByte/xvml:Accel2",
    qname = "xsd:integer/xvml:Row",
    qname = "xsd:integer/xvml:Column",
    qname = "xvml:ST_TrueFalseBlank/xvml:Visible",
    qname = "xvml:ST_TrueFalseBlank/xvml:RowHidden",
    qname = "xvml:ST_TrueFalseBlank/xvml:ColHidden",
    qname = "xsd:integer/xvml:VTEdit",
    qname = "xvml:ST_TrueFalseBlank/xvml:MultiLine",
    qname = "xvml:ST_TrueFalseBlank/xvml:VScroll",
    qname = "xvml:ST_TrueFalseBlank/xvml:ValidIds",
    qname = "xsd:string/xvml:FmlaRange",
    qname = "xsd:integer/xvml:WidthMin",
    qname = "xsd:integer/xvml:Sel",
    qname = "xvml:ST_TrueFalseBlank/xvml:NoThreeD2",
    qname = "xsd:string/xvml:SelType",
    qname = "xsd:string/xvml:MultiSel",
    qname = "xsd:string/xvml:LCT",
    qname = "xsd:string/xvml:ListItem",
    qname = "xsd:string/xvml:DropStyle",
    qname = "xvml:ST_TrueFalseBlank/xvml:Colored",
    qname = "xsd:integer/xvml:DropLines",
    qname = "xsd:integer/xvml:Checked",
    qname = "xsd:string/xvml:FmlaLink",
    qname = "xsd:string/xvml:FmlaPict",
    qname = "xvml:ST_TrueFalseBlank/xvml:NoThreeD",
    qname = "xvml:ST_TrueFalseBlank/xvml:FirstButton",
    qname = "xsd:string/xvml:FmlaGroup",
    qname = "xsd:integer/xvml:Val",
    qname = "xsd:integer/xvml:Min",
    qname = "xsd:integer/xvml:Max",
    qname = "xsd:integer/xvml:Inc",
    qname = "xsd:integer/xvml:Page",
    qname = "xvml:ST_TrueFalseBlank/xvml:Horiz",
    qname = "xsd:integer/xvml:Dx",
    qname = "xvml:ST_TrueFalseBlank/xvml:MapOCX",
    qname = "xvml:ST_CF/xvml:CF",
    qname = "xvml:ST_TrueFalseBlank/xvml:Camera",
    qname = "xvml:ST_TrueFalseBlank/xvml:RecalcAlways",
    qname = "xvml:ST_TrueFalseBlank/xvml:AutoScale",
    qname = "xvml:ST_TrueFalseBlank/xvml:DDE",
    qname = "xvml:ST_TrueFalseBlank/xvml:UIObj",
    qname = "xsd:string/xvml:ScriptText",
    qname = "xsd:string/xvml:ScriptExtended",
    qname = "xsd:nonNegativeInteger/xvml:ScriptLanguage",
    qname = "xsd:nonNegativeInteger/xvml:ScriptLocation",
    qname = "xsd:string/xvml:FmlaTxbx"
  ))]
  pub client_data_choice: Vec<ClientDataChoice>,
}
/// Move with Cells.
pub type MoveWithCells = BooleanEntryWithBlankValues;
/// Resize with Cells.
pub type ResizeWithCells = BooleanEntryWithBlankValues;
/// Lock Toggle.
pub type Locked = BooleanEntryWithBlankValues;
/// Default Size Toggle.
pub type DefaultSize = BooleanEntryWithBlankValues;
/// Print Toggle.
pub type PrintObject = BooleanEntryWithBlankValues;
/// Macro Disable Toggle.
pub type Disabled = BooleanEntryWithBlankValues;
/// AutoFill.
pub type AutoFill = BooleanEntryWithBlankValues;
/// AutoLine.
pub type AutoLine = BooleanEntryWithBlankValues;
/// Automatically Size.
pub type AutoSizePicture = BooleanEntryWithBlankValues;
/// Text Lock.
pub type LockText = BooleanEntryWithBlankValues;
/// East Asia Alignment Toggle.
pub type JustifyLastLine = BooleanEntryWithBlankValues;
/// Password Edit.
pub type SecretEdit = BooleanEntryWithBlankValues;
/// Default Button.
pub type DefaultButton = BooleanEntryWithBlankValues;
/// Help Button.
pub type HelpButton = BooleanEntryWithBlankValues;
/// Cancel Button.
pub type CancelButton = BooleanEntryWithBlankValues;
/// Dismiss Button.
pub type DismissButton = BooleanEntryWithBlankValues;
/// Comment Visibility Toggle.
pub type Visible = BooleanEntryWithBlankValues;
/// Comment's Row is Hidden.
pub type RowHidden = BooleanEntryWithBlankValues;
/// Comment's Column is Hidden.
pub type ColumnHidden = BooleanEntryWithBlankValues;
/// Multi-line.
pub type MultiLine = BooleanEntryWithBlankValues;
/// Vertical Scroll.
pub type VerticalScrollBar = BooleanEntryWithBlankValues;
/// Valid ID.
pub type ValidIds = BooleanEntryWithBlankValues;
/// Disable 3D.
pub type Disable3DForListBoxAndDropDown = BooleanEntryWithBlankValues;
/// Dropdown Color Toggle.
pub type Colored = BooleanEntryWithBlankValues;
/// Disable 3D.
pub type Disable3D = BooleanEntryWithBlankValues;
/// First Radio Button.
pub type FirstButton = BooleanEntryWithBlankValues;
/// Scroll Bar Orientation.
pub type HorizontalScrollBar = BooleanEntryWithBlankValues;
/// ActiveX Control.
pub type MapOcxControl = BooleanEntryWithBlankValues;
/// Camera Tool.
pub type CameraObject = BooleanEntryWithBlankValues;
/// Recalculation Toggle.
pub type RecalculateAlways = BooleanEntryWithBlankValues;
/// Font AutoScale.
pub type AutoScaleFont = BooleanEntryWithBlankValues;
/// Dynamic Data Exchange.
pub type DdeObject = BooleanEntryWithBlankValues;
/// UI Object Toggle.
pub type UiObject = BooleanEntryWithBlankValues;
/// Anchor.
pub type Anchor = crate::simple_type::StringValue;
/// Horizontal Text Alignment.
pub type HorizontalTextAlignment = crate::simple_type::StringValue;
/// Vertical Text Alignment.
pub type VerticalTextAlignment = crate::simple_type::StringValue;
/// List Items Source Range.
pub type FormulaRange = crate::simple_type::StringValue;
/// Selection Type.
pub type SelectionType = crate::simple_type::StringValue;
/// Multiple Selections.
pub type MultiSelections = crate::simple_type::StringValue;
/// Callback Type.
pub type ListBoxCallbackType = crate::simple_type::StringValue;
/// Non-linked List Item.
pub type ListItem = crate::simple_type::StringValue;
/// Dropdown Style.
pub type DropStyle = crate::simple_type::StringValue;
/// Linked Formula.
pub type FormulaLink = crate::simple_type::StringValue;
/// Camera Source Range.
pub type FormulaPicture = crate::simple_type::StringValue;
/// Linked Formula - Group Box.
pub type FormulaGroup = crate::simple_type::StringValue;
/// HTML Script Text.
pub type ScriptText = crate::simple_type::StringValue;
/// HTML Script Attributes.
pub type ScriptExtended = crate::simple_type::StringValue;
/// Text Formula.
pub type FormulaTextBox = crate::simple_type::StringValue;
/// Reference to Custom Function.
pub type FormulaMacro = crate::simple_type::StringValue;
/// Primary Keyboard Accelerator.
pub type AcceleratorPrimary = crate::simple_type::ByteValue;
/// Secondary Keyboard Accelerator.
pub type AcceleratorSecondary = crate::simple_type::ByteValue;
/// Comment Row Target.
pub type CommentRowTarget = crate::simple_type::IntegerValue;
/// Comment Column Target.
pub type CommentColumnTarget = crate::simple_type::IntegerValue;
/// Validation Type.
pub type InputValidationType = crate::simple_type::IntegerValue;
/// Minimum Width.
pub type MinDropDownWidth = crate::simple_type::IntegerValue;
/// Selected Entry.
pub type SelectionEntry = crate::simple_type::IntegerValue;
/// Dropdown Maximum Lines.
pub type DropLines = crate::simple_type::IntegerValue;
/// Checked.
pub type Checked = crate::simple_type::IntegerValue;
/// Scroll bar position.
pub type ScrollBarPosition = crate::simple_type::IntegerValue;
/// Scroll Bar Minimum.
pub type ScrollBarMin = crate::simple_type::IntegerValue;
/// Scroll Bar Maximum.
pub type ScrollBarMax = crate::simple_type::IntegerValue;
/// Scroll Bar Increment.
pub type ScrollBarIncrement = crate::simple_type::IntegerValue;
/// Scroll Bar Page Increment.
pub type ScrollBarPageIncrement = crate::simple_type::IntegerValue;
/// Scroll Bar Width.
pub type ScrollBarWidth = crate::simple_type::IntegerValue;
/// Clipboard Format.
pub type ClipboardFormat = ClipboardFormatValues;
/// HTML Script Language.
pub type ScriptLanguage = crate::simple_type::IntegerValue;
/// HTML Script Location.
pub type ScriptLocation = crate::simple_type::IntegerValue;
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ClientDataChoice {
  /// Move with Cells.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:MoveWithCells"))]
  XvmlMoveWithCells(BooleanEntryWithBlankValues),
  /// Resize with Cells.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:SizeWithCells"))]
  XvmlSizeWithCells(BooleanEntryWithBlankValues),
  /// Anchor.
  #[sdk(text_child(qname = "xsd:string/xvml:Anchor"))]
  XvmlAnchor(crate::simple_type::StringValue),
  /// Lock Toggle.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Locked"))]
  XvmlLocked(BooleanEntryWithBlankValues),
  /// Default Size Toggle.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:DefaultSize"))]
  XvmlDefaultSize(BooleanEntryWithBlankValues),
  /// Print Toggle.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:PrintObject"))]
  XvmlPrintObject(BooleanEntryWithBlankValues),
  /// Macro Disable Toggle.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Disabled"))]
  XvmlDisabled(BooleanEntryWithBlankValues),
  /// AutoFill.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:AutoFill"))]
  XvmlAutoFill(BooleanEntryWithBlankValues),
  /// AutoLine.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:AutoLine"))]
  XvmlAutoLine(BooleanEntryWithBlankValues),
  /// Automatically Size.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:AutoPict"))]
  XvmlAutoPict(BooleanEntryWithBlankValues),
  /// Reference to Custom Function.
  #[sdk(text_child(qname = "xvml:ST_Macro/xvml:FmlaMacro"))]
  XvmlFmlaMacro(crate::simple_type::StringValue),
  /// Horizontal Text Alignment.
  #[sdk(text_child(qname = "xsd:string/xvml:TextHAlign"))]
  XvmlTextHAlign(crate::simple_type::StringValue),
  /// Vertical Text Alignment.
  #[sdk(text_child(qname = "xsd:string/xvml:TextVAlign"))]
  XvmlTextVAlign(crate::simple_type::StringValue),
  /// Text Lock.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:LockText"))]
  XvmlLockText(BooleanEntryWithBlankValues),
  /// East Asia Alignment Toggle.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:JustLastX"))]
  XvmlJustLastX(BooleanEntryWithBlankValues),
  /// Password Edit.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:SecretEdit"))]
  XvmlSecretEdit(BooleanEntryWithBlankValues),
  /// Default Button.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Default"))]
  XvmlDefault(BooleanEntryWithBlankValues),
  /// Help Button.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Help"))]
  XvmlHelp(BooleanEntryWithBlankValues),
  /// Cancel Button.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Cancel"))]
  XvmlCancel(BooleanEntryWithBlankValues),
  /// Dismiss Button.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Dismiss"))]
  XvmlDismiss(BooleanEntryWithBlankValues),
  /// Primary Keyboard Accelerator.
  #[sdk(text_child(qname = "xsd:unsignedByte/xvml:Accel"))]
  XvmlAccel(crate::simple_type::ByteValue),
  /// Secondary Keyboard Accelerator.
  #[sdk(text_child(qname = "xsd:unsignedByte/xvml:Accel2"))]
  XvmlAccel2(crate::simple_type::ByteValue),
  /// Comment Row Target.
  #[sdk(text_child(qname = "xsd:integer/xvml:Row"))]
  XvmlRow(crate::simple_type::IntegerValue),
  /// Comment Column Target.
  #[sdk(text_child(qname = "xsd:integer/xvml:Column"))]
  XvmlColumn(crate::simple_type::IntegerValue),
  /// Comment Visibility Toggle.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Visible"))]
  XvmlVisible(BooleanEntryWithBlankValues),
  /// Comment's Row is Hidden.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:RowHidden"))]
  XvmlRowHidden(BooleanEntryWithBlankValues),
  /// Comment's Column is Hidden.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:ColHidden"))]
  XvmlColHidden(BooleanEntryWithBlankValues),
  /// Validation Type.
  #[sdk(text_child(qname = "xsd:integer/xvml:VTEdit"))]
  XvmlVtEdit(crate::simple_type::IntegerValue),
  /// Multi-line.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:MultiLine"))]
  XvmlMultiLine(BooleanEntryWithBlankValues),
  /// Vertical Scroll.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:VScroll"))]
  XvmlVScroll(BooleanEntryWithBlankValues),
  /// Valid ID.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:ValidIds"))]
  XvmlValidIds(BooleanEntryWithBlankValues),
  /// List Items Source Range.
  #[sdk(text_child(qname = "xsd:string/xvml:FmlaRange"))]
  XvmlFmlaRange(crate::simple_type::StringValue),
  /// Minimum Width.
  #[sdk(text_child(qname = "xsd:integer/xvml:WidthMin"))]
  XvmlWidthMin(crate::simple_type::IntegerValue),
  /// Selected Entry.
  #[sdk(text_child(qname = "xsd:integer/xvml:Sel"))]
  XvmlSel(crate::simple_type::IntegerValue),
  /// Disable 3D.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:NoThreeD2"))]
  XvmlNoThreeD2(BooleanEntryWithBlankValues),
  /// Selection Type.
  #[sdk(text_child(qname = "xsd:string/xvml:SelType"))]
  XvmlSelType(crate::simple_type::StringValue),
  /// Multiple Selections.
  #[sdk(text_child(qname = "xsd:string/xvml:MultiSel"))]
  XvmlMultiSel(crate::simple_type::StringValue),
  /// Callback Type.
  #[sdk(text_child(qname = "xsd:string/xvml:LCT"))]
  XvmlLct(crate::simple_type::StringValue),
  /// Non-linked List Item.
  #[sdk(text_child(qname = "xsd:string/xvml:ListItem"))]
  XvmlListItem(crate::simple_type::StringValue),
  /// Dropdown Style.
  #[sdk(text_child(qname = "xsd:string/xvml:DropStyle"))]
  XvmlDropStyle(crate::simple_type::StringValue),
  /// Dropdown Color Toggle.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Colored"))]
  XvmlColored(BooleanEntryWithBlankValues),
  /// Dropdown Maximum Lines.
  #[sdk(text_child(qname = "xsd:integer/xvml:DropLines"))]
  XvmlDropLines(crate::simple_type::IntegerValue),
  /// Checked.
  #[sdk(text_child(qname = "xsd:integer/xvml:Checked"))]
  XvmlChecked(crate::simple_type::IntegerValue),
  /// Linked Formula.
  #[sdk(text_child(qname = "xsd:string/xvml:FmlaLink"))]
  XvmlFmlaLink(crate::simple_type::StringValue),
  /// Camera Source Range.
  #[sdk(text_child(qname = "xsd:string/xvml:FmlaPict"))]
  XvmlFmlaPict(crate::simple_type::StringValue),
  /// Disable 3D.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:NoThreeD"))]
  XvmlNoThreeD(BooleanEntryWithBlankValues),
  /// First Radio Button.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:FirstButton"))]
  XvmlFirstButton(BooleanEntryWithBlankValues),
  /// Linked Formula - Group Box.
  #[sdk(text_child(qname = "xsd:string/xvml:FmlaGroup"))]
  XvmlFmlaGroup(crate::simple_type::StringValue),
  /// Scroll bar position.
  #[sdk(text_child(qname = "xsd:integer/xvml:Val"))]
  XvmlVal(crate::simple_type::IntegerValue),
  /// Scroll Bar Minimum.
  #[sdk(text_child(qname = "xsd:integer/xvml:Min"))]
  XvmlMin(crate::simple_type::IntegerValue),
  /// Scroll Bar Maximum.
  #[sdk(text_child(qname = "xsd:integer/xvml:Max"))]
  XvmlMax(crate::simple_type::IntegerValue),
  /// Scroll Bar Increment.
  #[sdk(text_child(qname = "xsd:integer/xvml:Inc"))]
  XvmlInc(crate::simple_type::IntegerValue),
  /// Scroll Bar Page Increment.
  #[sdk(text_child(qname = "xsd:integer/xvml:Page"))]
  XvmlPage(crate::simple_type::IntegerValue),
  /// Scroll Bar Orientation.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Horiz"))]
  XvmlHoriz(BooleanEntryWithBlankValues),
  /// Scroll Bar Width.
  #[sdk(text_child(qname = "xsd:integer/xvml:Dx"))]
  XvmlDx(crate::simple_type::IntegerValue),
  /// ActiveX Control.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:MapOCX"))]
  XvmlMapOcx(BooleanEntryWithBlankValues),
  /// Clipboard Format.
  #[sdk(text_child(qname = "xvml:ST_CF/xvml:CF"))]
  XvmlCf(ClipboardFormatValues),
  /// Camera Tool.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Camera"))]
  XvmlCamera(BooleanEntryWithBlankValues),
  /// Recalculation Toggle.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:RecalcAlways"))]
  XvmlRecalcAlways(BooleanEntryWithBlankValues),
  /// Font AutoScale.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:AutoScale"))]
  XvmlAutoScale(BooleanEntryWithBlankValues),
  /// Dynamic Data Exchange.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:DDE"))]
  XvmlDde(BooleanEntryWithBlankValues),
  /// UI Object Toggle.
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:UIObj"))]
  XvmlUiObj(BooleanEntryWithBlankValues),
  /// HTML Script Text.
  #[sdk(text_child(qname = "xsd:string/xvml:ScriptText"))]
  XvmlScriptText(crate::simple_type::StringValue),
  /// HTML Script Attributes.
  #[sdk(text_child(qname = "xsd:string/xvml:ScriptExtended"))]
  XvmlScriptExtended(crate::simple_type::StringValue),
  /// HTML Script Language.
  #[sdk(text_child(qname = "xsd:nonNegativeInteger/xvml:ScriptLanguage"))]
  XvmlScriptLanguage(crate::simple_type::IntegerValue),
  /// HTML Script Location.
  #[sdk(text_child(qname = "xsd:nonNegativeInteger/xvml:ScriptLocation"))]
  XvmlScriptLocation(crate::simple_type::IntegerValue),
  /// Text Formula.
  #[sdk(text_child(qname = "xsd:string/xvml:FmlaTxbx"))]
  XvmlFmlaTxbx(crate::simple_type::StringValue),
}
