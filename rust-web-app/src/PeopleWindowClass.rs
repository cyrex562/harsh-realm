// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PeopleWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class PeopleWindowClass : WindowClass
  {
     LibListId: i32;
     LibNr: i32;
     ListClass LibListObj;
     peopleListId: i32;
     ListClass peopleListObj;
     BAddpeopleId: i32;
     BAddpeopleTextId: i32;
     BNameId: i32;
     BNameTextId: i32;
     B1Id: i32;
     B1TextId: i32;
     highId: i32;
     lowId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B4bId: i32;
     B4bTextId: i32;
     B4cId: i32;
     B4cTextId: i32;
     z1id: i32;
     z1textid: i32;
     BRemovepeopleId: i32;
     BRemovepeopleTextId: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     firstListId: i32;
     ListClass firstListObj;
     lastListId: i32;
     ListClass lastListObj;
     CombatListId: i32;
     ListClass CombatListObj;
     b19id: i32;
     b19textid: i32;
     b20id: i32;
     b20textid: i32;
     b18id: i32;
     b18textid: i32;
     b21id: i32;
     b21textid: i32;
     b22id: i32;
     b22textid: i32;
     b23id: i32;
     b23textid: i32;
     b24id: i32;
     b24textid: i32;
     b25id: i32;
     b25textid: i32;
     b26id: i32;
     b26textid: i32;
     b27id: i32;
     b27textid: i32;
     b28id: i32;
     b28textid: i32;
     a1Id: i32;
     a1TextId: i32;
     a2Id: i32;
     a2TextId: i32;
     a3Id: i32;
     a3TextId: i32;
     a4Id: i32;
     a4TextId: i32;
     a5Id: i32;
     a5TextId: i32;
     a6Id: i32;
     a6TextId: i32;
     a7Id: i32;
     a7TextId: i32;
     a8Id: i32;
     a8TextId: i32;
     x1id: i32;
     x2id: i32;
     x3id: i32;
     x4id: i32;
     x5id: i32;
     x6id: i32;
     x7id: i32;
     x8id: i32;
     x9id: i32;
     x10id: i32;
     x11id: i32;
     x12id: i32;
     killnameid: i32;
     addnameid: i32;
     addtext1: i32;
     killname2id: i32;
     addname2id: i32;
     addtext2: i32;
     c1id: i32;
     OffPicId: i32;
     c2id: i32;
     c2textid: i32;
     peopleNr: i32;
     detailnr: i32;
     firstdetail: i32;
     lastdetail: i32;
     offnr: i32;
     ss: String;

    pub PeopleWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Peoples")
    {
      this.peopleNr = -1;
      this.detailnr = -1;
      this.LibNr = -1;
      this.firstdetail = -1;
      this.lastdetail = -1;
      this.offnr = -1;
      this.MakepeopleListGUI(-1);
    }

    pub fn DoRefresh() => this.MakepeopleTypeItemGUI();

     void MakepeopleListGUI(tpeoplenr: i32)
    {
      if (this.LibListId > 0)
        this.RemoveSubPart(this.LibListId);
      this.LibListObj = ListClass::new();
      this.LibListObj.add("All", -2);
      let mut num1: i32 =  -1;
      let mut num2: i32 =  0;
      let mut libraryCounter: i32 =  this.game.Data.LibraryCounter;
      for (let mut index: i32 =  0; index <= libraryCounter; index += 1)
      {
        num2 += 1;
        if (this.LibNr == index)
          num1 = num2;
        this.LibListObj.add(Conversion.Str( index) + ") " + this.game.Data.LibraryObj[index].name, index);
      }
      if (this.LibNr == -1)
        num1 = 0;
      ListClass libListObj = this.LibListObj;
      let mut tlistselect1: i32 =  num1;
      let mut game1: GameClass = this.game;
       local1: Bitmap =  this.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(libListObj, 9, 200, tlistselect1, game1, tHeader: "Libraries", tbackbitmap: ( local1), bbx: 10, bby: 38, overruleFont: ( local2));
      this.LibListId = this.AddSubPart( tsubpart1, 10, 50, 200, 192, 0);
      if (this.peopleListId > 0)
        this.RemoveSubPart(this.peopleListId);
      let mut num3: i32 =  -1;
      let mut num4: i32 =  -1;
      if (this.game.Data.PeopleCounter > -1)
      {
        this.peopleListObj = ListClass::new();
        let mut peopleCounter: i32 =  this.game.Data.PeopleCounter;
        for (let mut index: i32 =  0; index <= peopleCounter; index += 1)
        {
          if (this.game.Data.PeopleObj[index].LibId.libSlot == this.LibNr | this.LibNr == -1)
          {
            num4 += 1;
            if (this.LibNr == index)
              num3 = num4;
            this.peopleListObj.add(Conversion.Str( index) + ") " + this.game.Data.PeopleObj[index].Name + "(id" + this.game.Data.PeopleObj[index].id.ToString() + ")", index);
          }
        }
        ListClass peopleListObj = this.peopleListObj;
        let mut tlistselect2: i32 =  num3;
        let mut game2: GameClass = this.game;
         local3: Bitmap =  this.OwnBitmap;
        font =  null;
         local4: Font =  font;
        let mut tsubpart2: SubPartClass =  new ListSubPartClass(peopleListObj, 9, 200, tlistselect2, game2, tHeader: "Peoples", tbackbitmap: ( local3), bbx: 220, bby: 50, overruleFont: ( local4));
        this.peopleListId = this.AddSubPart( tsubpart2, 220, 50, 200, 192, 0);
        this.peopleNr = tpeoplenr;
        this.MakepeopleTypeItemGUI();
      }
      else
      {
        this.peopleNr = tpeoplenr;
        this.MakepeopleTypeItemGUI();
      }
      if (this.BAddpeopleId > 0)
        this.RemoveSubPart(this.BAddpeopleId);
      if (this.BAddpeopleTextId > 0)
        this.RemoveSubPart(this.BAddpeopleTextId);
      if (this.z1id > 0)
        this.RemoveSubPart(this.z1id);
      if (this.z1textid > 0)
        this.RemoveSubPart(this.z1textid);
      this.ss = "Click to add a people to the list";
      SubPartClass tsubpart3;
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
        this.BAddpeopleId = this.AddSubPart( tsubpart3, 10, 270, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("Add a People", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.BAddpeopleTextId = this.AddSubPart( tsubpart3, 50, 269, 300, 20, 0);
      this.ss = "Click to set all units of a specific regime to a specific people";
      tsubpart3 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.z1id = this.AddSubPart( tsubpart3, 410, 290, 32, 16, 1);
      tsubpart3 =  TextPartClass::new("Set Units of Regime X", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.z1textid = this.AddSubPart( tsubpart3, 450, 289, 300, 20, 0);
    }

     void MakepeopleTypeItemGUI()
    {
      if (this.BNameId > 0)
        this.RemoveSubPart(this.BNameId);
      if (this.BNameTextId > 0)
        this.RemoveSubPart(this.BNameTextId);
      if (this.B1Id > 0)
        this.RemoveSubPart(this.B1Id);
      if (this.B1TextId > 0)
        this.RemoveSubPart(this.B1TextId);
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
      if (this.B4bId > 0)
        this.RemoveSubPart(this.B4bId);
      if (this.B4bTextId > 0)
        this.RemoveSubPart(this.B4bTextId);
      if (this.B4cId > 0)
        this.RemoveSubPart(this.B4cId);
      if (this.B4cTextId > 0)
        this.RemoveSubPart(this.B4cTextId);
      if (this.highId > 0)
        this.RemoveSubPart(this.highId);
      if (this.lowId > 0)
        this.RemoveSubPart(this.lowId);
      if (this.b18id > 0)
        this.RemoveSubPart(this.b18id);
      if (this.b18textid > 0)
        this.RemoveSubPart(this.b18textid);
      if (this.b19id > 0)
        this.RemoveSubPart(this.b19id);
      if (this.b19textid > 0)
        this.RemoveSubPart(this.b19textid);
      if (this.b20id > 0)
        this.RemoveSubPart(this.b20id);
      if (this.b20textid > 0)
        this.RemoveSubPart(this.b20textid);
      if (this.b21id > 0)
        this.RemoveSubPart(this.b21id);
      if (this.b21textid > 0)
        this.RemoveSubPart(this.b21textid);
      if (this.b22id > 0)
        this.RemoveSubPart(this.b22id);
      if (this.b22textid > 0)
        this.RemoveSubPart(this.b22textid);
      if (this.b23id > 0)
        this.RemoveSubPart(this.b23id);
      if (this.b23textid > 0)
        this.RemoveSubPart(this.b23textid);
      if (this.b24id > 0)
        this.RemoveSubPart(this.b24id);
      if (this.b24textid > 0)
        this.RemoveSubPart(this.b24textid);
      if (this.b25id > 0)
        this.RemoveSubPart(this.b25id);
      if (this.b25textid > 0)
        this.RemoveSubPart(this.b25textid);
      if (this.b26id > 0)
        this.RemoveSubPart(this.b26id);
      if (this.b26textid > 0)
        this.RemoveSubPart(this.b26textid);
      if (this.b27id > 0)
        this.RemoveSubPart(this.b27id);
      if (this.b27textid > 0)
        this.RemoveSubPart(this.b27textid);
      if (this.b28id > 0)
        this.RemoveSubPart(this.b28id);
      if (this.b28textid > 0)
        this.RemoveSubPart(this.b28textid);
      if (this.a1Id > 0)
        this.RemoveSubPart(this.a1Id);
      if (this.a1TextId > 0)
        this.RemoveSubPart(this.a1TextId);
      if (this.a2Id > 0)
        this.RemoveSubPart(this.a2Id);
      if (this.a2TextId > 0)
        this.RemoveSubPart(this.a2TextId);
      if (this.a3Id > 0)
      {
        this.RemoveSubPart(this.a3Id);
        this.a3Id = 0;
      }
      if (this.a3TextId > 0)
        this.RemoveSubPart(this.a3TextId);
      if (this.a4Id > 0)
        this.RemoveSubPart(this.a4Id);
      if (this.a4TextId > 0)
        this.RemoveSubPart(this.a4TextId);
      if (this.a5Id > 0)
        this.RemoveSubPart(this.a5Id);
      if (this.a5TextId > 0)
        this.RemoveSubPart(this.a5TextId);
      if (this.a6Id > 0)
        this.RemoveSubPart(this.a6Id);
      if (this.a6TextId > 0)
        this.RemoveSubPart(this.a6TextId);
      if (this.a7Id > 0)
        this.RemoveSubPart(this.a7Id);
      if (this.a7TextId > 0)
        this.RemoveSubPart(this.a7TextId);
      if (this.a8Id > 0)
        this.RemoveSubPart(this.a8Id);
      if (this.a8TextId > 0)
        this.RemoveSubPart(this.a8TextId);
      if (this.c1id > 0)
        this.RemoveSubPart(this.c1id);
      if (this.OffPicId > 0)
        this.RemoveSubPart(this.OffPicId);
      if (this.c2id > 0)
        this.RemoveSubPart(this.c2id);
      if (this.c2textid > 0)
        this.RemoveSubPart(this.c2textid);
      if (this.killnameid > 0)
        this.RemoveSubPart(this.killnameid);
      if (this.addnameid > 0)
        this.RemoveSubPart(this.addnameid);
      if (this.killname2id > 0)
        this.RemoveSubPart(this.killname2id);
      if (this.addname2id > 0)
        this.RemoveSubPart(this.addname2id);
      if (this.BRemovepeopleId > 0)
        this.RemoveSubPart(this.BRemovepeopleId);
      if (this.BRemovepeopleTextId > 0)
        this.RemoveSubPart(this.BRemovepeopleTextId);
      if (this.firstListId > 0)
        this.RemoveSubPart(this.firstListId);
      if (this.lastListId > 0)
        this.RemoveSubPart(this.lastListId);
      if (this.CombatListId > 0)
        this.RemoveSubPart(this.CombatListId);
      if (this.addtext1 > 0)
        this.RemoveSubPart(this.addtext1);
      if (this.addtext2 > 0)
        this.RemoveSubPart(this.addtext2);
      if (this.x1id > 0)
        this.RemoveSubPart(this.x1id);
      if (this.x2id > 0)
        this.RemoveSubPart(this.x2id);
      if (this.x3id > 0)
        this.RemoveSubPart(this.x3id);
      if (this.x4id > 0)
        this.RemoveSubPart(this.x4id);
      if (this.x5id > 0)
        this.RemoveSubPart(this.x5id);
      if (this.x6id > 0)
        this.RemoveSubPart(this.x6id);
      if (this.x7id > 0)
        this.RemoveSubPart(this.x7id);
      if (this.x8id > 0)
        this.RemoveSubPart(this.x8id);
      if (this.x9id > 0)
        this.RemoveSubPart(this.x9id);
      if (this.x10id > 0)
        this.RemoveSubPart(this.x10id);
      if (this.x11id > 0)
        this.RemoveSubPart(this.x11id);
      if (this.x12id > 0)
        this.RemoveSubPart(this.x12id);
      if (this.peopleNr <= -1)
        return;
      this.ss = "Click to change the name of this people";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BNameId = this.AddSubPart( tsubpart, 500, 50, 32, 16, 1);
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Name: " + this.game.Data.PeopleObj[this.peopleNr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.BNameTextId = this.AddSubPart( tsubpart1, 540, 49, 200, 20, 0);
      this.ss = "Click to change the colour used to pacounters: i32 with these people in them. -1,-1,-1= dont use special colour. Using this, instead of regcol makes the graphics slightly slower.";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B1Id = this.AddSubPart( tsubpart2, 800, 50, 32, 16, 1);
      }
      let mut tsubpart3: SubPartClass =  TextPartClass::new("Colour: " + Conversion.Str( this.game.Data.PeopleObj[this.peopleNr].Red) + "," + Conversion.Str( this.game.Data.PeopleObj[this.peopleNr].Green) + "," + Conversion.Str( this.game.Data.PeopleObj[this.peopleNr].Blue), Font::new("Times New Roman", 12f), 200, 20, false);
      this.B1TextId = this.AddSubPart( tsubpart3, 840, 49, 200, 20, 0);
      this.ss = "Cancel the using of a special overruling colour for this people.";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart4: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
        this.B2Id = this.AddSubPart( tsubpart4, 800, 70, 32, 16, 1);
      }
      let mut tsubpart5: SubPartClass =  TextPartClass::new("Kill own people colour", Font::new("Times New Roman", 12f), 200, 20, false);
      this.B2TextId = this.AddSubPart( tsubpart5, 840, 69, 200, 20, 0);
      this.ss = "Select Library for people";
      str1: String = "";
      if (this.game.Data.PeopleObj[this.peopleNr].LibId.libSlot > -1)
        str1 += this.game.Data.PeopleObj[this.peopleNr].LibId.libSlot.ToString();
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart6: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B4Id = this.AddSubPart( tsubpart6, 800, 110, 32, 16, 1);
      }
      let mut tsubpart7: SubPartClass =  TextPartClass::new("Set Library (" + str1 + ")", Font::new("Times New Roman", 12f), 200, 20, false);
      this.B4TextId = this.AddSubPart( tsubpart7, 840, 109, 200, 20, 0);
      this.ss = "EXPERT USE ONLY - ";
      str2: String = "";
      if (this.game.Data.PeopleObj[this.peopleNr].LibId.libSlot > -1)
        str2 += this.game.Data.PeopleObj[this.peopleNr].LibId.id.ToString();
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart8: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B4bId = this.AddSubPart( tsubpart8, 800, 130, 32, 16, 1);
      }
      let mut tsubpart9: SubPartClass =  TextPartClass::new("Set Library ID (" + str2 + ")", Font::new("Times New Roman", 12f), 200, 20, false);
      this.B4bTextId = this.AddSubPart( tsubpart9, 840, 129, 200, 20, 0);
      this.ss = "EXPERT USE ONLY - ";
      str3: String = "" + this.game.Data.PeopleObj[this.peopleNr].id.ToString();
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart10: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B4cId = this.AddSubPart( tsubpart10, 800, 150, 32, 16, 1);
      }
      let mut tsubpart11: SubPartClass =  TextPartClass::new("Set ID (" + str3 + ")", Font::new("Times New Roman", 12f), 200, 20, false);
      this.B4cTextId = this.AddSubPart( tsubpart11, 840, 149, 200, 20, 0);
      if (this.peopleNr < this.game.Data.PeopleCounter)
      {
        let mut tsubpart12: SubPartClass =  ButtonPartClass::new(this.game.BUTTONDOWN, tDescript: "Move down list");
        this.highId = this.AddSubPart( tsubpart12, 750, 130, 32, 16, 1);
      }
      if (this.peopleNr > 0)
      {
        let mut tsubpart13: SubPartClass =  ButtonPartClass::new(this.game.BUTTONUP, tDescript: "Move Up List");
        this.lowId = this.AddSubPart( tsubpart13, 750, 110, 32, 16, 1);
      }
      this.ss = "Click to change the peoplegroup this people belong too";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart14: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B3Id = this.AddSubPart( tsubpart14, 500, 90, 32, 16, 1);
      }
      let mut tsubpart15: SubPartClass =  TextPartClass::new("PeopleGroup: " + this.game.Data.TempString[200 + this.game.Data.PeopleObj[this.peopleNr].PeopleGroup] + " (" + Conversion.Str( this.game.Data.PeopleObj[this.peopleNr].PeopleGroup) + ")", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.B3TextId = this.AddSubPart( tsubpart15, 540, 89, 200, 20, 0);
      this.ss = "Click to change the break percentage of troops of this people";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart15 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.b25id = this.AddSubPart( tsubpart15, 500, 130, 32, 16, 1);
      }
      tsubpart15 =  TextPartClass::new("BreakAt: " + Conversion.Str( this.game.Data.PeopleObj[this.peopleNr].BreakAt), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.b25textid = this.AddSubPart( tsubpart15, 540, 129, 200, 20, 0);
      this.ss = "Click to change the People Gfx for this People (for sideways gfx)";
      tsubpart15 =  ButtonPartClass::new(this.game.Data.PeopleObj[this.peopleNr].SidewaysSpriteID, tDescript: this.ss);
      this.x1id = this.AddSubPart( tsubpart15, 570, 180, 140, 80, 1);
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart15 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.x2id = this.AddSubPart( tsubpart15, 530, 180, 32, 16, 1);
      }
      this.ss = "Click to change the overrule symbol for this People (national sprite)";
      tsubpart15 =  ButtonPartClass::new(this.game.Data.PeopleObj[this.peopleNr].NationalSpriteID, tDescript: this.ss);
      this.x3id = this.AddSubPart( tsubpart15, 770, 180, 37, 37, 1);
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart15 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.x4id = this.AddSubPart( tsubpart15, 910, 180, 32, 16, 1);
      }
      if (this.game.Data.Product >= 7)
      {
        this.ss = "Click to change the People Gfx for this People (for sideways gfx)";
        tsubpart15 =  ButtonPartClass::new(this.game.Data.PeopleObj[this.peopleNr].SidewaysSpriteID, tDescript: this.ss);
        this.x5id = this.AddSubPart( tsubpart15, 970, 180, 140, 80, 1);
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart15 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.x6id = this.AddSubPart( tsubpart15, 930, 180, 32, 16, 1);
        }
        this.ss = "Click to change the People Gfx for this People (for sideways gfx)";
        tsubpart15 =  ButtonPartClass::new(this.game.Data.PeopleObj[this.peopleNr].SidewaysSpriteID, tDescript: this.ss);
        this.x7id = this.AddSubPart( tsubpart15, 1170, 180, 140, 80, 1);
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart15 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.x8id = this.AddSubPart( tsubpart15, 1130, 180, 32, 16, 1);
        }
        this.ss = "Click to change the People Gfx for this People (for sideways gfx)";
        tsubpart15 =  ButtonPartClass::new(this.game.Data.PeopleObj[this.peopleNr].SidewaysSpriteID, tDescript: this.ss);
        this.x9id = this.AddSubPart( tsubpart15, 1370, 180, 140, 80, 1);
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart15 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.x10id = this.AddSubPart( tsubpart15, 1330, 180, 32, 16, 1);
        }
      }
      if (this.game.Data.PeopleCounter > 0)
      {
        this.ss = "Click to remove this people";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart15 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
          this.BRemovepeopleId = this.AddSubPart( tsubpart15, 10, 290, 32, 16, 1);
        }
        tsubpart15 =  TextPartClass::new("remove a People", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.BRemovepeopleTextId = this.AddSubPart( tsubpart15, 40, 289, 300, 20, 0);
      }
      this.CombatListObj = ListClass::new();
      if (this.detailnr < -1 | this.detailnr > 99)
        this.detailnr = -1;
      let mut index: i32 =  0;
      do
      {
        str4: String = "";
        str5: String = Conversion.Str( index) + ") " + this.game.Data.TempString[index + 200];
        if (Strings.Len(str5) > 23)
          str5 = Strings.Left(str5, 23);
        str6: String = str4 + str5 + Strings.Space(25 - Strings.Len(str5));
        Expression1: String = "BMr=" + Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.peopleNr].BaseMorale[index]));
        str7: String = str6 + Expression1 + Strings.Space(15 - Strings.Len(Expression1));
        Expression2: String = "BFr=" + Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.peopleNr].BattleForMod[index]));
        str8: String = str7 + Expression2 + Strings.Space(15 - Strings.Len(Expression2));
        Expression3: String = "BVS=" + Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.peopleNr].BattleVSMod[index]));
        this.CombatListObj.add(str8 + Expression3 + Strings.Space(15 - Strings.Len(Expression3)), index);
        index += 1;
      }
      while (index <= 99);
      ListClass combatListObj = this.CombatListObj;
      let mut detailnr: i32 =  this.detailnr;
      let mut game: GameClass = this.game;
       local1: Bitmap =  this.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      tsubpart15 =  new ListSubPartClass(combatListObj, 12, 580, detailnr, game, true, "Owned by Peoplegroup modifiers", tbackbitmap: ( local1), bbx: 10, bby: 340, overruleFont: ( local2));
      this.CombatListId = this.AddSubPart( tsubpart15, 10, 340, 580, 240, 0);
      if (this.detailnr <= -1)
        return;
      this.maketabsheetnr4b();
    }

    pub fn maketabsheetnr4b()
    {
      this.ss = "Click to set basemorale this people have if ruled by a regime of selected peoplegroup";
      str1: String = Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.peopleNr].BaseMorale[this.detailnr]));
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.b18id = this.AddSubPart( tsubpart, 610, 340, 32, 16, 1);
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new("BaseMorale: " + str1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.b18textid = this.AddSubPart( tsubpart1, 650, 339, 400, 20, 0);
      this.ss = "Click to set combat modifier this people have if ruled by a regime of selected peoplegroup. 1=no mod. 1.5=50% better. 0.5=50% worse";
      str2: String = Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.peopleNr].BattleForMod[this.detailnr]));
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.b20id = this.AddSubPart( tsubpart2, 610, 380, 32, 16, 1);
      }
      let mut tsubpart3: SubPartClass =  TextPartClass::new("BattleFor: " + str2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.b20textid = this.AddSubPart( tsubpart3, 650, 379, 400, 20, 0);
      this.ss = "Click to set combat modifier this people have if they fight against a subformation of selected peoplegroup. 1=no mod. 1.5=50% better. 0.5=50% worse";
      str3: String = Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.peopleNr].BattleVSMod[this.detailnr]));
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart4: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.b21id = this.AddSubPart( tsubpart4, 610, 400, 32, 16, 1);
      }
      let mut tsubpart5: SubPartClass =  TextPartClass::new("BattleVS: " + str3, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.b21textid = this.AddSubPart( tsubpart5, 650, 399, 400, 20, 0);
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
                this.LibNr = num2;
                this.peopleNr = -1;
                this.MakepeopleListGUI(this.peopleNr);
              }
              else if (num2 == -2)
              {
                this.LibNr = -1;
                this.peopleNr = -1;
                this.MakepeopleListGUI(this.peopleNr);
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.peopleListId)
            {
              let mut num3: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num3 > -1)
              {
                this.peopleNr = num3;
                this.MakepeopleTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.firstListId)
            {
              let mut num4: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num4 > -1)
              {
                this.firstdetail = num4;
                this.MakepeopleTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.x2id)
            {
              filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp|Jpg|*.jpg", "Give File Name For sideways gfx", this.game.AppPath + "graphics\\", true);
              if (File.Exists(this.game.AppPath + "graphics/" + filename))
              {
                this.game.Data.PeopleObj[this.peopleNr].ReplaceSidewaysSprite(filename);
              }
              else
              {
                let mut num5: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.x4id)
            {
              filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp|Jpg|*.jpg", "Give File Name For symbol overrule gfx", this.game.AppPath + "graphics\\", true);
              if (File.Exists(this.game.AppPath + "graphics/" + filename))
              {
                this.game.Data.PeopleObj[this.peopleNr].ReplaceNationalSprite(filename);
              }
              else
              {
                let mut num6: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.x6id)
            {
              filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp|Jpg|*.jpg", "Give File Name For sideways gfx 2", this.game.AppPath + "graphics\\", true);
              if (File.Exists(this.game.AppPath + "graphics/" + filename))
              {
                this.game.Data.PeopleObj[this.peopleNr].ReplaceSidewaysSprite2(filename);
              }
              else
              {
                let mut num7: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.x8id)
            {
              filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp|Jpg|*.jpg", "Give File Name For sideways gfx 3", this.game.AppPath + "graphics\\", true);
              if (File.Exists(this.game.AppPath + "graphics/" + filename))
              {
                this.game.Data.PeopleObj[this.peopleNr].ReplaceSidewaysSprite3(filename);
              }
              else
              {
                let mut num8: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.x10id)
            {
              filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp|Jpg|*.jpg", "Give File Name For sideways gfx 4", this.game.AppPath + "graphics\\", true);
              if (File.Exists(this.game.AppPath + "graphics/" + filename))
              {
                this.game.Data.PeopleObj[this.peopleNr].ReplaceSidewaysSprite4(filename);
              }
              else
              {
                let mut num9: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.lastListId)
            {
              let mut num10: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num10 > -1)
              {
                this.lastdetail = num10;
                this.MakepeopleTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BAddpeopleId)
            {
              this.game.Data.AddPeople();
              this.game.Data.PeopleObj[this.game.Data.PeopleCounter].LibId.libSlot = this.LibNr;
              if (this.peopleNr > -1 & Interaction.MsgBox( "Insert it in current selected people slot?", MsgBoxStyle.YesNo) == MsgBoxResult.Yes)
              {
                PeopleClass peopleClass = this.game.Data.PeopleObj[this.game.Data.PeopleCounter];
                let mut peopleCounter: i32 =  this.game.Data.PeopleCounter;
                let mut num11: i32 =  this.peopleNr + 1;
                for (let mut Newnr: i32 =  peopleCounter; Newnr >= num11; Newnr += -1)
                {
                  this.game.Data.PeopleObj[Newnr] = this.game.Data.PeopleObj[Newnr - 1];
                  this.game.Data.ChangePeopleNr(Newnr - 1, Newnr);
                }
                this.game.Data.PeopleObj[this.peopleNr] = peopleClass;
              }
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BNameId)
            {
              this.game.Data.PeopleObj[this.peopleNr].Name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B1Id)
            {
              ColorDialog colorDialog = ColorDialog::new();
              colorDialog.Color = this.game.Data.PeopleObj[this.peopleNr].Red <= -1 ? Color.FromArgb( byte.MaxValue, 128, 128, 128) : Color.FromArgb( byte.MaxValue, this.game.Data.PeopleObj[this.peopleNr].Red, this.game.Data.PeopleObj[this.peopleNr].Green, this.game.Data.PeopleObj[this.peopleNr].Blue);
              if (colorDialog.ShowDialog() == DialogResult.OK)
              {
                PeopleClass peopleClass1 = this.game.Data.PeopleObj[this.peopleNr];
                color: Color = colorDialog.Color;
                let mut r: i32 =   color.R;
                peopleClass1.Red = r;
                PeopleClass peopleClass2 = this.game.Data.PeopleObj[this.peopleNr];
                color = colorDialog.Color;
                let mut g: i32 =   color.G;
                peopleClass2.Green = g;
                PeopleClass peopleClass3 = this.game.Data.PeopleObj[this.peopleNr];
                color = colorDialog.Color;
                let mut b1: i32 =   color.B;
                peopleClass3.Blue = b1;
              }
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B2Id)
            {
              this.game.Data.PeopleObj[this.peopleNr].Red = -1;
              this.game.Data.PeopleObj[this.peopleNr].Green = -1;
              this.game.Data.PeopleObj[this.peopleNr].Blue = -1;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B4Id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 96, this.peopleNr);
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.highId)
            {
              this.game.EditObj.UnitSelected = -1;
              this.game.Data.MovePeopleHigher(this.peopleNr);
              this += 1.peopleNr;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.lowId)
            {
              this.game.EditObj.UnitSelected = -1;
              this.game.Data.MovePeopleLower(this.peopleNr);
              --this.peopleNr;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B3Id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 13, this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BRemovepeopleId)
            {
              this.game.Data.RemovePeople(this.peopleNr);
              this.MakepeopleListGUI(-1);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.CombatListId)
            {
              let mut num12: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num12 > -1)
              {
                this.detailnr = num12;
                this.MakepeopleListGUI(this.peopleNr);
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b23id)
            {
              Form2::new( this.formref).Initialize(this.game.Data, 11, this.peopleNr);
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b18id)
            {
              let mut num13: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give new base morale.", "Shadow Empire : Planetary Conquest")));
              if (num13 < 0 | num13 > 100)
              {
                let mut num14: i32 =   Interaction.MsgBox( "Between 0 and 100 please. ", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.PeopleObj[this.peopleNr].BaseMorale[this.detailnr] = num13;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B4bId)
            {
              this.game.Data.PeopleObj[this.peopleNr].LibId.id =  Math.Round(Conversion.Val(Interaction.InputBox("Give new library ID.", "Shadow Empire : Planetary Conquest")));
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B4cId)
            {
              let mut num15: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give new ID.", "Shadow Empire : Planetary Conquest")));
              this.game.Data.PeopleObj[this.peopleNr].id = num15;
              if (num15 > this.game.Data.PeopleIdCounter)
                this.game.Data.PeopleIdCounter = num15;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.a6Id)
            {
              let mut num16: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give new extra graphic (-1=none).", "Shadow Empire : Planetary Conquest")));
              if (num16 < -1 | num16 > 100)
              {
                let mut num17: i32 =   Interaction.MsgBox( "Between -1 and 100 please. ", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.PeopleObj[this.peopleNr].ExtraGraphicUse = num16;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.z1id)
            {
              let mut num18: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give regime# to use.", "Shadow Empire : Planetary Conquest")));
              if (num18 < 0 | num18 > this.game.Data.RegimeCounter)
                return windowReturnClass;
              let mut num19: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give people# to use.", "Shadow Empire : Planetary Conquest")));
              if (num19 < 0 | num19 > this.game.Data.PeopleCounter)
                return windowReturnClass;
              let mut unitCounter: i32 =  this.game.Data.UnitCounter;
              for (let mut index2: i32 =  0; index2 <= unitCounter; index2 += 1)
              {
                if (this.game.Data.UnitObj[index2].PreDef == -1 & this.game.Data.UnitObj[index2].Regime == num18)
                {
                  let mut sfCount: i32 =  this.game.Data.UnitObj[index2].SFCount;
                  for (let mut index3: i32 =  0; index3 <= sfCount; index3 += 1)
                    this.game.Data.SFObj[this.game.Data.UnitObj[index2].SFList[index3]].People = num19;
                }
              }
              let mut num20: i32 =   Interaction.MsgBox( "Done! ", Title: ( "Shadow Empire : Planetary Conquest"));
              return windowReturnClass;
            }
            if (num1 == this.a7Id)
            {
              let mut num21: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give new SFType # to use.", "Shadow Empire : Planetary Conquest")));
              if (num21 < -1 | num21 > this.game.Data.SFTypeCounter)
              {
                let mut num22: i32 =   Interaction.MsgBox( "Between -1 and SFTypeCounter please. ", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.PeopleObj[this.peopleNr].SFIll = num21;
              this.game.Data.PeopleObj[this.peopleNr].SFExtra =  Math.Round(Conversion.Val(Interaction.InputBox("Give extra graphic # to use.", "Shadow Empire : Planetary Conquest")));
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b25id)
            {
              let mut num23: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give new break At %. 0=never break. 50=chance starts from half unit lost. 100=immediate chance.", "Shadow Empire : Planetary Conquest")));
              if (num23 < 0 | num23 > 100)
              {
                let mut num24: i32 =   Interaction.MsgBox( "Between 0 and 100 please. ", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.PeopleObj[this.peopleNr].BreakAt = num23;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b22id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 14, this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b19id)
            {
              float num25 =  Conversion.Val(Interaction.InputBox("Give Prod Mod.", "Shadow Empire : Planetary Conquest"));
              if ( num25 < 0.0 |  num25 > 10.0)
              {
                let mut num26: i32 =   Interaction.MsgBox( "Between 0 and 10 please. You can use 0.51 or 4.87 like values.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.PeopleObj[this.peopleNr].ProdMod[this.detailnr] = num25;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b28id)
            {
              float num27 =  Conversion.Val(Interaction.InputBox("Give Prod Mod 4.", "Shadow Empire : Planetary Conquest"));
              if ( num27 < 0.0 |  num27 > 10.0)
              {
                let mut num28: i32 =   Interaction.MsgBox( "Between 0 and 10 please. You can use 0.51 or 4.87 like values.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.PeopleObj[this.peopleNr].ProdMod4[this.detailnr] = num27;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b27id)
            {
              float num29 =  Conversion.Val(Interaction.InputBox("Give Prod Mod 3.", "Shadow Empire : Planetary Conquest"));
              if ( num29 < 0.0 |  num29 > 10.0)
              {
                let mut num30: i32 =   Interaction.MsgBox( "Between 0 and 10 please. You can use 0.51 or 4.87 like values.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.PeopleObj[this.peopleNr].ProdMod3[this.detailnr] = num29;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b26id)
            {
              float num31 =  Conversion.Val(Interaction.InputBox("Give Prod Mod 2.", "Shadow Empire : Planetary Conquest"));
              if ( num31 < 0.0 |  num31 > 10.0)
              {
                let mut num32: i32 =   Interaction.MsgBox( "Between 0 and 10 please. You can use 0.51 or 4.87 like values.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.PeopleObj[this.peopleNr].ProdMod2[this.detailnr] = num31;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b20id)
            {
              float num33 =  Conversion.Val(Interaction.InputBox("Give Battle For Mod (normal=1).", "Shadow Empire : Planetary Conquest"));
              if ( num33 < 0.0 |  num33 > 10.0)
              {
                let mut num34: i32 =   Interaction.MsgBox( "Between 0 and 10 please. You can use 0.51 or 4.87 like values.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.PeopleObj[this.peopleNr].BattleForMod[this.detailnr] = num33;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b21id)
            {
              float num35 =  Conversion.Val(Interaction.InputBox("Give Battle VS Mod (normal=1).", "Shadow Empire : Planetary Conquest"));
              if ( num35 < 0.0 |  num35 > 10.0)
              {
                let mut num36: i32 =   Interaction.MsgBox( "Between 0 and 10 please. You can use 0.51 or 4.87 like values.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.PeopleObj[this.peopleNr].BattleVSMod[this.detailnr] = num35;
              this.MakepeopleListGUI(this.peopleNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
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
