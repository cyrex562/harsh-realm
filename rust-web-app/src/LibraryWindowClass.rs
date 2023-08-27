// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LibraryWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class LibraryWindowClass : WindowClass
  {
     LibListId: i32;
     ListClass LibListObj;
     AddLibId: i32;
     AddLibTextId: i32;
     RemoveLibId: i32;
     RemoveLibTextId: i32;
     LibVarListId: i32;
     ListClass LibVarListObj;
     AddLibVarId: i32;
     AddLibVarTextId: i32;
     RemoveLibVarId: i32;
     RemoveLibVarTextId: i32;
     LibVarTypeId: i32;
     LibVarTypeTextId: i32;
     LibVarNameId: i32;
     LibVarNameTextId: i32;
     LibVarInfoId: i32;
     LibVarInfoTextId: i32;
     LibVarValueTypeId: i32;
     LibVarValueTypeTextId: i32;
     BNameId: i32;
     BNameTextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     LibId: i32;
     LibVarId: i32;
     ss: String;

    pub LibraryWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Libraries")
    {
      this.LibId = -1;
      this.LibVarId = -1;
      this.MakeLibListGUI(-1);
    }

    pub fn DoRefresh() => this.MakeLibItemGUI();

     void MakeLibListGUI(tLibNr: i32)
    {
      if (this.LibListId > 0)
        this.RemoveSubPart(this.LibListId);
      this.LibListObj = ListClass::new();
      tLibNr = -1;
      let mut libraryCounter: i32 =  this.game.Data.LibraryCounter;
      for (let mut index: i32 =  0; index <= libraryCounter; index += 1)
      {
        this.LibListObj.add(Conversion.Str( index) + ") " + this.game.Data.LibraryObj[index].name, index);
        if (this.LibId == index)
          tLibNr = index + 1;
      }
      if (tLibNr == -1)
        this.LibId = -1;
      ListClass libListObj = this.LibListObj;
      let mut libId: i32 =  this.LibId;
      let mut game: GameClass = this.game;
       local1: Bitmap =  this.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(libListObj, 19, 400, libId, game, tHeader: "Libraries", tbackbitmap: ( local1), bbx: 10, bby: 50, overruleFont: ( local2));
      this.LibListId = this.AddSubPart( tsubpart1, 10, 50, 400, 352, 0);
      this.MakeLibItemGUI();
      if (this.AddLibId > 0)
        this.RemoveSubPart(this.AddLibId);
      if (this.AddLibTextId > 0)
        this.RemoveSubPart(this.AddLibTextId);
      this.ss = "Click to add a new Library";
      let mut tsubpart2: SubPartClass =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
      this.AddLibId = this.AddSubPart( tsubpart2, 10, 430, 32, 16, 1);
      let mut tsubpart3: SubPartClass =  TextPartClass::new("Add Library", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.AddLibTextId = this.AddSubPart( tsubpart3, 50, 429, 300, 20, 0);
    }

     void MakeLibItemGUI()
    {
      if (this.LibVarListId > 0)
        this.RemoveSubPart(this.LibVarListId);
      if (this.BNameId > 0)
        this.RemoveSubPart(this.BNameId);
      if (this.BNameTextId > 0)
        this.RemoveSubPart(this.BNameTextId);
      if (this.RemoveLibId > 0)
        this.RemoveSubPart(this.RemoveLibId);
      if (this.RemoveLibTextId > 0)
        this.RemoveSubPart(this.RemoveLibTextId);
      if (this.AddLibVarId > 0)
        this.RemoveSubPart(this.AddLibVarId);
      if (this.AddLibVarTextId > 0)
        this.RemoveSubPart(this.AddLibVarTextId);
      if (this.B2Id > 0)
        this.RemoveSubPart(this.B2Id);
      if (this.B2TextId > 0)
        this.RemoveSubPart(this.B2TextId);
      if (this.B3Id > 0)
        this.RemoveSubPart(this.B3Id);
      if (this.B3TextId > 0)
        this.RemoveSubPart(this.B3TextId);
      if (this.B4Id > 0)
        this.RemoveSubPart(this.B4Id);
      if (this.B4TextId > 0)
        this.RemoveSubPart(this.B4TextId);
      if (this.B5Id > 0)
        this.RemoveSubPart(this.B5Id);
      if (this.B5TextId > 0)
        this.RemoveSubPart(this.B5TextId);
      if (this.LibId > -1)
      {
        this.ss = "Click to change the name of this Library. Make sure its a unique name. So maybe use part of your name or something thats not easily thought of by other designer.";
        let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BNameId = this.AddSubPart( tsubpart1, 470, 50, 32, 16, 1);
        let mut tsubpart2: SubPartClass =  TextPartClass::new("name: " + this.game.Data.LibraryObj[this.LibId].name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.BNameTextId = this.AddSubPart( tsubpart2, 510, 49, 400, 20, 0);
        this.ss = "Click to change the name of this Library. Make sure its a unique name. So maybe use part of your name or something thats not easily thought of by other designer.";
        let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B2Id = this.AddSubPart( tsubpart3, 470, 80, 32, 16, 1);
        let mut tsubpart4: SubPartClass =  TextPartClass::new("creator: " + this.game.Data.LibraryObj[this.LibId].creator, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.B2TextId = this.AddSubPart( tsubpart4, 510, 79, 400, 20, 0);
        this.ss = "Click to document how to use your library.";
        let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B3Id = this.AddSubPart( tsubpart5, 470, 110, 32, 16, 1);
        let mut tsubpart6: SubPartClass =  TextPartClass::new("information: " + Strings.Left(this.game.Data.LibraryObj[this.LibId].information, 20) + "...", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.B3TextId = this.AddSubPart( tsubpart6, 510, 109, 400, 20, 0);
        this.ss = "Click to change the name of this Library. Make sure its a unique name. So maybe use part of your name or something thats not easily thought of by other designer.";
        let mut tsubpart7: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B4Id = this.AddSubPart( tsubpart7, 470, 140, 32, 16, 1);
        let mut tsubpart8: SubPartClass =  TextPartClass::new("version: " + this.game.Data.LibraryObj[this.LibId].version.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.B4TextId = this.AddSubPart( tsubpart8, 510, 139, 400, 20, 0);
        this.ss = "Click to create a single event library.";
        let mut tsubpart9: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B5Id = this.AddSubPart( tsubpart9, 470, 170, 32, 16, 1);
        let mut tsubpart10: SubPartClass =  TextPartClass::new("Export Single Library", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.B5TextId = this.AddSubPart( tsubpart10, 510, 169, 400, 20, 0);
        this.ss = "Click to remove this Library.";
        let mut tsubpart11: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
        this.RemoveLibId = this.AddSubPart( tsubpart11, 10, 450, 32, 16, 1);
        let mut tsubpart12: SubPartClass =  TextPartClass::new("Remove Library", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.RemoveLibTextId = this.AddSubPart( tsubpart12, 50, 449, 200, 20, 0);
        if (this.LibVarListId > 0)
          this.RemoveSubPart(this.LibVarListId);
        let mut num1: i32 =  -1;
        let mut num2: i32 =  -1;
        this.LibVarListObj = ListClass::new();
        let mut libVarCounter: i32 =  this.game.Data.LibVarCounter;
        for (let mut index: i32 =  0; index <= libVarCounter; index += 1)
        {
          if (this.game.Data.LibVarObj[index].libId.libSlot == this.LibId)
          {
            num2 += 1;
            this.LibVarListObj.add(Conversion.Str( index) + ") " + this.game.Data.LibVarObj[index].name, index);
            if (this.LibVarId == index)
              num1 = num2;
          }
        }
        if (num1 == -1)
          this.LibVarId = -1;
        ListClass libVarListObj = this.LibVarListObj;
        let mut tlistselect: i32 =  num1;
        let mut game: GameClass = this.game;
         local1: Bitmap =  this.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart13: SubPartClass =  new ListSubPartClass(libVarListObj, 14, 400, tlistselect, game, tHeader: "LibVars", tbackbitmap: ( local1), bbx: 470, bby: 200, overruleFont: ( local2));
        this.LibVarListId = this.AddSubPart( tsubpart13, 470, 200, 400, 272, 0);
        if (this.AddLibVarId > 0)
          this.RemoveSubPart(this.AddLibVarId);
        if (this.AddLibVarTextId > 0)
          this.RemoveSubPart(this.AddLibVarTextId);
        this.ss = "Click to add a new LibVar";
        let mut tsubpart14: SubPartClass =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
        this.AddLibVarId = this.AddSubPart( tsubpart14, 470, 500, 32, 16, 1);
        let mut tsubpart15: SubPartClass =  TextPartClass::new("Add LibVar", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.AddLibVarTextId = this.AddSubPart( tsubpart15, 520, 499, 300, 20, 0);
      }
      this.MakeLibVarItemGUI();
    }

     void MakeLibVarItemGUI()
    {
      if (this.LibVarTypeId > 0)
        this.RemoveSubPart(this.LibVarTypeId);
      if (this.LibVarValueTypeId > 0)
        this.RemoveSubPart(this.LibVarValueTypeId);
      if (this.LibVarNameId > 0)
        this.RemoveSubPart(this.LibVarNameId);
      if (this.LibVarTypeTextId > 0)
        this.RemoveSubPart(this.LibVarTypeTextId);
      if (this.LibVarValueTypeTextId > 0)
        this.RemoveSubPart(this.LibVarValueTypeTextId);
      if (this.LibVarNameTextId > 0)
        this.RemoveSubPart(this.LibVarNameTextId);
      if (this.LibVarInfoId > 0)
        this.RemoveSubPart(this.LibVarInfoId);
      if (this.LibVarInfoTextId > 0)
        this.RemoveSubPart(this.LibVarInfoTextId);
      if (this.RemoveLibVarId > 0)
        this.RemoveSubPart(this.RemoveLibVarId);
      if (this.RemoveLibVarTextId > 0)
        this.RemoveSubPart(this.RemoveLibVarTextId);
      if (this.LibVarId <= -1)
        return;
      this.ss = "";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.LibVarTypeId = this.AddSubPart( tsubpart1, 470, 550, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("Type: " + this.game.Data.LibVarObj[this.LibVarId].type.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.LibVarTypeTextId = this.AddSubPart( tsubpart2, 510, 549, 400, 20, 0);
      this.ss = "";
      tsubpart2 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.LibVarNameId = this.AddSubPart( tsubpart2, 470, 580, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Name: " + this.game.Data.LibVarObj[this.LibVarId].name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.LibVarNameTextId = this.AddSubPart( tsubpart2, 510, 579, 400, 20, 0);
      if (this.game.Data.LibVarObj[this.LibVarId].type != NewEnums.LibVarType.Hex)
      {
        this.ss = "";
        tsubpart2 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.LibVarValueTypeId = this.AddSubPart( tsubpart2, 470, 610, 32, 16, 1);
        tsubpart2 =  TextPartClass::new("ValueType: " + this.game.Data.LibVarObj[this.LibVarId].valueType.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.LibVarValueTypeTextId = this.AddSubPart( tsubpart2, 510, 609, 400, 20, 0);
      }
      this.ss = "";
      tsubpart2 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.LibVarInfoId = this.AddSubPart( tsubpart2, 470, 640, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Information: " + Strings.Left(this.game.Data.LibVarObj[this.LibVarId].information, 20) + "...", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.LibVarInfoTextId = this.AddSubPart( tsubpart2, 510, 639, 400, 20, 0);
      this.ss = "Click to remove this libvar.";
      tsubpart2 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
      this.RemoveLibVarId = this.AddSubPart( tsubpart2, 470, 520, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Remove Libvar", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.RemoveLibVarTextId = this.AddSubPart( tsubpart2, 520, 519, 200, 20, 0);
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.LibListId)
            {
              let mut num2: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                this.LibId = num2;
                this.LibVarId = -1;
                this.MakeLibItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.LibVarListId)
            {
              let mut num3: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num3 > -1)
              {
                this.LibVarId = num3;
                this.MakeLibVarItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.AddLibId)
            {
              this.game.Data.AddLibrary();
              this.MakeLibListGUI(this.LibId);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.AddLibVarId)
            {
              this.game.Data.AddLibVar(this.LibId);
              this.MakeLibItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.RemoveLibVarId)
            {
              this.game.Data.RemoveLibVar(this.LibVarId);
              this.MakeLibItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BNameId)
            {
              this.game.Data.LibraryObj[this.LibId].name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
              this.MakeLibListGUI(this.LibId);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B2Id)
            {
              this.game.Data.LibraryObj[this.LibId].creator = Interaction.InputBox("Give creator name, please.", "Shadow Empire : Planetary Conquest");
              this.MakeLibListGUI(this.LibId);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B3Id)
            {
              Form2::new( this.formref).Initialize(this.game.Data, 13, this.LibId);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B5Id)
            {
              if (this.LibId < 0)
                return windowReturnClass;
              tinitdir: String = this.game.AppPath + "scenarios\\";
              if (!Information.IsNothing( this.game.Data.ScenarioDir))
              {
                if (this.game.Data.ScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", this.game.Data.ScenarioDir);
                else if (this.game.ModScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", this.game.ModScenarioDir);
              }
              else if (this.game.ModScenarioDir.Length > 1)
                tinitdir = tinitdir.Replace("scenarios", this.game.ModScenarioDir);
              str: String = this.game.HandyFunctionsObj.SaveSomething("SE1 Event library(*.se1evlib)|*.se1evlib", "Give save name for event library...", tinitdir, false);
              this.game.FormRef.Cursor = Cursors.WaitCursor;
              dataClass: DataClass = this.game.Data.Clone();
              name: String = this.game.Data.LibraryObj[this.LibId].name;
              for (let mut regimeCounter: i32 =  dataClass.RegimeCounter; regimeCounter >= 0; regimeCounter += -1)
                dataClass.RemoveRegime(regimeCounter);
              for (let mut locCounter: i32 =  dataClass.LocCounter; locCounter >= 0; locCounter += -1)
                dataClass.RemoveLoc(locCounter);
              int[] numArray = new int[dataClass.SmallPicCounter + 1];
              let mut smallPicCounter1: i32 =  dataClass.SmallPicCounter;
              for (let mut index2: i32 =  0; index2 <= smallPicCounter1; index2 += 1)
                numArray[index2] = index2;
              let mut smallPicCounter2: i32 =  dataClass.SmallPicCounter;
              for (let mut nr: i32 =  0; nr <= smallPicCounter2 && dataClass.SmallPicCounter >= 0 && nr <= dataClass.SmallPicCounter; nr += 1)
              {
                if (dataClass.SmallLibId[nr].libSlot != this.LibId)
                {
                  let mut num4: i32 =  nr;
                  let mut num5: i32 =  dataClass.SmallPicCounter - 1;
                  for (let mut index3: i32 =  num4; index3 <= num5; index3 += 1)
                    numArray[index3] = numArray[index3 + 1];
                  dataClass.RemoveSmallPic(nr);
                  if (dataClass.SmallPicCounter > -1)
                    --nr;
                }
              }
              let mut smallPicCounter3: i32 =  dataClass.SmallPicCounter;
              for (let mut index4: i32 =  0; index4 <= smallPicCounter3; index4 += 1)
                this.game.Data.SmallLibId[numArray[index4]].id = index4;
              for (let mut libraryCounter: i32 =  dataClass.LibraryCounter; libraryCounter >= 0; libraryCounter += -1)
              {
                if (Operators.CompareString(dataClass.LibraryObj[libraryCounter].name, name, false) != 0)
                  dataClass.RemoveLibrary(libraryCounter);
              }
              let mut eventCounter: i32 =  dataClass.EventCounter;
              for (let mut index5: i32 =  0; index5 <= eventCounter; index5 += 1)
              {
                if (dataClass.EventObj[index5].LibId.libSlot == 0 && dataClass.EventObj[index5].LibId.id > -1)
                {
                  dataClass.EventObj[index5].Id = dataClass.EventObj[index5].LibId.id;
                  dataClass.EventObj[index5].LibId.id = -1;
                }
              }
              let mut stringListCounter: i32 =  dataClass.StringListCounter;
              for (let mut index6: i32 =  0; index6 <= stringListCounter; index6 += 1)
              {
                if (dataClass.StringListObj[index6].LibId.libSlot == 0 && dataClass.StringListObj[index6].LibId.id > -1)
                {
                  dataClass.StringListObj[index6].ID = dataClass.StringListObj[index6].LibId.id;
                  dataClass.StringListObj[index6].LibId.id = -1;
                }
              }
              let mut eventPicCounter: i32 =  dataClass.EventPicCounter;
              for (let mut index7: i32 =  0; index7 <= eventPicCounter; index7 += 1)
              {
                if (dataClass.eventPicLibId[index7].libSlot == 0 && dataClass.eventPicLibId[index7].id > -1)
                {
                  dataClass.EventPicNr[index7] = dataClass.eventPicLibId[index7].id;
                  dataClass.eventPicLibId[index7].id = -1;
                }
              }
              dataClass.MasterFile = "";
              dataClass.Description = "Event library";
              dataClass.Name = name + " event library";
              dataClass.serialize(str);
              this.game.HandyFunctionsObj.ZipFile(str);
              windowReturnClass.SetFlag(true);
              this.game.FormRef.Cursor = Cursors.Default;
              let mut num6: i32 =   Interaction.MsgBox( "Saved", Title: ( "Shadow Empire : Planetary Conquest"));
            }
            else
            {
              if (num1 == this.LibVarNameId)
              {
                this.game.Data.LibVarObj[this.LibVarId].name = Interaction.InputBox("Give name, please.", "Shadow Empire : Planetary Conquest");
                this.MakeLibItemGUI();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.LibVarInfoId)
              {
                Form2::new( this.formref).Initialize(this.game.Data, 14, this.LibVarId);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.LibVarValueTypeId)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 92, this.LibVarId);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.LibVarTypeId)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 91, this.LibVarId);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B4Id)
              {
                let mut num7: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give version number please.", "Shadow Empire : Planetary Conquest")));
                if (num7 >= 1 & num7 <= 9999)
                {
                  this.game.Data.LibraryObj[this.LibId].version = num7;
                }
                else
                {
                  let mut num8: i32 =   Interaction.MsgBox( "Value between 1-9999 please.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                this.MakeLibListGUI(this.LibId);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.RemoveLibId)
              {
                this.game.Data.RemoveLibrary(this.LibId);
                if (this.LibId > this.game.Data.LibraryCounter)
                  this.LibId = this.game.Data.LibraryCounter;
                this.LibVarId = -1;
                this.MakeLibListGUI(this.LibId);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
