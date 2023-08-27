// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.EditUnitWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class EditUnitWindowClass : WindowClass
  {
     LibListId: i32;
     LibNr: i32;
     ListClass LibListObj;
     UnitListId: i32;
     ListClass UnitListObj;
     BAddUnitId: i32;
     BAddUnitTextId: i32;
     UnitList2Id: i32;
     ListClass UnitList2Obj;
     BAddUnit2Id: i32;
     BAddUnit2TextId: i32;
     BNameId: i32;
     BNameTextId: i32;
     BRemoveUnitId: i32;
     BRemoveUnitTextId: i32;
     SFListId: i32;
     ListClass SFListObj;
     BAddSFId: i32;
     BaddSFTextId: i32;
     BRemoveSFId: i32;
     BRemoveSFTextid: i32;
     BHQId: i32;
     BHQTextId: i32;
     a3id: i32;
     a3bid: i32;
     a4id: i32;
     a4bid: i32;
     a5id: i32;
     a5bid: i32;
     a6id: i32;
     a6bid: i32;
     a7id: i32;
     a7bid: i32;
     a8id: i32;
     a8bid: i32;
     a9id: i32;
     a9bid: i32;
     a10id: i32;
     a10bid: i32;
     a11id: i32;
     a11bid: i32;
     BTypeId: i32;
     BTypeTextId: i32;
     SFTypeListId: i32;
     ListClass SFTypeListObj;
     BQtyID: i32;
     BQtyTextId: i32;
     BRdnId: i32;
     BRdnTextId: i32;
     BXpId: i32;
     BXpTextId: i32;
     BPeopleId: i32;
     BPeopleTextId: i32;
     BSupplyId: i32;
     BSupplyTextId: i32;
     BMorId: i32;
     BMorTextId: i32;
     bmovid: i32;
     bmovtextid: i32;
     BEntrId: i32;
     BEntrTextId: i32;
     UnitNr: i32;
     SFNr: i32;
     SFTypeNr: i32;
     ss: String;

    pub EditUnitWindowClass(ref tGame: GameClass)
      : base(ref tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Units in Hex and Units Predefined ")
    {
      this.UnitNr = -1;
      this.LibNr = -1;
      this.SFNr = -1;
      this.SFTypeNr = -1;
      this.MakeUnitListGUI(-1);
    }

    pub fn DoRefresh() => this.MakeUnitTypeItemGUI();

     void MakeUnitListGUI(tUnitnr: i32)
    {
      if (this.UnitListId > 0)
        this.RemoveSubPart(this.UnitListId);
      if (this.LibListId > 0)
        this.RemoveSubPart(this.LibListId);
      this.UnitNr = tUnitnr;
      let mut num1: i32 =  -1;
      font: Font;
      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > -1)
      {
        this.UnitListObj = ListClass::new();
        let mut unitCounter: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter;
        for (let mut index: i32 =  0; index <= unitCounter; index += 1)
        {
          let mut unit: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitList[index];
          if (unit == tUnitnr)
            num1 = index;
          this.UnitListObj.add("-" + this.game.Data.UnitObj[unit].Name + "(#" + unit.ToString() + ")", unit);
        }
        ListClass unitListObj = this.UnitListObj;
        let mut tlistselect: i32 =  num1;
        let mut game: GameClass = this.game;
        ref local1: Bitmap = ref this.OwnBitmap;
        font =  null;
        ref local2: Font = ref font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(unitListObj, 3, 300, tlistselect, game, tHeader: "Units in Hex", tbackbitmap: (ref local1), bbx: 10, bby: 55, overruleFont: (ref local2));
        this.UnitListId = this.AddSubPart(ref tsubpart, 10, 55, 300, 96, 0);
        if (num1 > -1)
          this.MakeUnitTypeItemGUI();
      }
      else if (num1 > -1)
        this.MakeUnitTypeItemGUI();
      this.LibListObj = ListClass::new();
      this.LibListObj.add("All", -2);
      let mut num2: i32 =  -1;
      let mut num3: i32 =  0;
      let mut libraryCounter: i32 =  this.game.Data.LibraryCounter;
      for (let mut index: i32 =  0; index <= libraryCounter; index += 1)
      {
        num3 += 1;
        if (this.LibNr == index)
          num2 = num3;
        this.LibListObj.add(Conversion.Str( index) + ") " + this.game.Data.LibraryObj[index].name, index);
      }
      if (this.LibNr == -1)
        num2 = 0;
      ListClass libListObj = this.LibListObj;
      let mut tlistselect1: i32 =  num2;
      let mut game1: GameClass = this.game;
      ref local3: Bitmap = ref this.OwnBitmap;
      font =  null;
      ref local4: Font = ref font;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(libListObj, 5, 200, tlistselect1, game1, tHeader: "Libraries", tbackbitmap: (ref local3), bbx: 10, bby: 180, overruleFont: (ref local4));
      this.LibListId = this.AddSubPart(ref tsubpart1, 10, 180, 300, 128, 0);
      if (this.UnitList2Id > 0)
        this.RemoveSubPart(this.UnitList2Id);
      this.UnitNr = tUnitnr;
      let mut num4: i32 =  -1;
      this.UnitList2Obj = ListClass::new();
      let mut num5: i32 =  -1;
      let mut unitCounter1: i32 =  this.game.Data.UnitCounter;
      for (let mut index1: i32 =  0; index1 <= unitCounter1; index1 += 1)
      {
        if (this.game.Data.UnitObj[index1].LibId.libSlot == this.LibNr | this.LibNr == -1)
        {
          let mut index2: i32 =  index1;
          if (this.game.Data.UnitObj[index2].PreDef > -1)
          {
            num5 += 1;
            if (index2 == tUnitnr)
              num4 = num5;
            this.UnitList2Obj.add("#" + Strings.Trim(Conversion.Str( this.game.Data.UnitObj[index2].PreDef)) + ") " + this.game.Data.UnitObj[index2].Name + " (" + Conversion.Str( index2) + ")", index2);
          }
        }
      }
      ListClass unitList2Obj = this.UnitList2Obj;
      let mut tlistselect2: i32 =  num4;
      let mut game2: GameClass = this.game;
      ref local5: Bitmap = ref this.OwnBitmap;
      font =  null;
      ref local6: Font = ref font;
      let mut tsubpart2: SubPartClass =  new ListSubPartClass(unitList2Obj, 5, 300, tlistselect2, game2, tHeader: "Predefined Units", tbackbitmap: (ref local5), bbx: 10, bby: 324, overruleFont: (ref local6));
      this.UnitList2Id = this.AddSubPart(ref tsubpart2, 10, 324, 300, 128, 0);
      this.MakeUnitTypeItemGUI();
      if (this.BAddUnitId > 0)
        this.RemoveSubPart(this.BAddUnitId);
      if (this.BAddUnitTextId > 0)
        this.RemoveSubPart(this.BAddUnitTextId);
      if (this.BAddUnit2Id > 0)
        this.RemoveSubPart(this.BAddUnit2Id);
      if (this.BAddUnit2TextId > 0)
        this.RemoveSubPart(this.BAddUnit2TextId);
      this.ss = "Click to add a unit in this hex";
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
      this.BAddUnitId = this.AddSubPart(ref tsubpart3, 10, 480, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("Add Unit in Hex", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.BAddUnitTextId = this.AddSubPart(ref tsubpart4, 50, 479, 300, 20, 0);
      this.ss = "Click to add a predefined unit. These can be used by events.";
      let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
      this.BAddUnit2Id = this.AddSubPart(ref tsubpart5, 10, 500, 32, 16, 1);
      let mut tsubpart6: SubPartClass =  TextPartClass::new("Add PreDef Unit", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.BAddUnit2TextId = this.AddSubPart(ref tsubpart6, 50, 499, 300, 20, 0);
    }

     void MakeUnitTypeItemGUI()
    {
      if (this.BNameId > 0)
        this.RemoveSubPart(this.BNameId);
      if (this.BNameTextId > 0)
        this.RemoveSubPart(this.BNameTextId);
      if (this.BRemoveUnitId > 0)
        this.RemoveSubPart(this.BRemoveUnitId);
      if (this.BRemoveUnitTextId > 0)
        this.RemoveSubPart(this.BRemoveUnitTextId);
      if (this.SFListId > 0)
        this.RemoveSubPart(this.SFListId);
      if (this.BAddSFId > 0)
        this.RemoveSubPart(this.BAddSFId);
      if (this.BaddSFTextId > 0)
        this.RemoveSubPart(this.BaddSFTextId);
      if (this.BRemoveSFId > 0)
        this.RemoveSubPart(this.BRemoveSFId);
      if (this.BRemoveSFTextid > 0)
        this.RemoveSubPart(this.BRemoveSFTextid);
      if (this.BHQId > 0)
        this.RemoveSubPart(this.BHQId);
      if (this.BHQTextId > 0)
        this.RemoveSubPart(this.BHQTextId);
      if (this.BSupplyId > 0)
        this.RemoveSubPart(this.BSupplyId);
      if (this.BSupplyTextId > 0)
        this.RemoveSubPart(this.BSupplyTextId);
      if (this.a3id > 0)
        this.RemoveSubPart(this.a3id);
      if (this.a3bid > 0)
        this.RemoveSubPart(this.a3bid);
      if (this.a4id > 0)
        this.RemoveSubPart(this.a4id);
      if (this.a4bid > 0)
        this.RemoveSubPart(this.a4bid);
      if (this.a5id > 0)
        this.RemoveSubPart(this.a5id);
      if (this.a5bid > 0)
        this.RemoveSubPart(this.a5bid);
      if (this.a6id > 0)
        this.RemoveSubPart(this.a6id);
      if (this.a6bid > 0)
        this.RemoveSubPart(this.a6bid);
      if (this.a7id > 0)
        this.RemoveSubPart(this.a7id);
      if (this.a7bid > 0)
        this.RemoveSubPart(this.a7bid);
      if (this.a8id > 0)
        this.RemoveSubPart(this.a8id);
      if (this.a8bid > 0)
        this.RemoveSubPart(this.a8bid);
      if (this.a9id > 0)
        this.RemoveSubPart(this.a9id);
      if (this.a9bid > 0)
        this.RemoveSubPart(this.a9bid);
      if (this.a10id > 0)
        this.RemoveSubPart(this.a10id);
      if (this.a10bid > 0)
        this.RemoveSubPart(this.a10bid);
      if (this.a11id > 0)
        this.RemoveSubPart(this.a11id);
      if (this.a11bid > 0)
        this.RemoveSubPart(this.a11bid);
      this.ss = "Click to import predef units from another file";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
      this.a8id = this.AddSubPart(ref tsubpart1, 10, 540, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("Import PreDef Units", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a8bid = this.AddSubPart(ref tsubpart2, 50, 539, 300, 20, 0);
      this.ss = "Click to delete all units (you can make exception for predef units)";
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
      this.a10id = this.AddSubPart(ref tsubpart3, 10, 560, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("Delete all units", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a10bid = this.AddSubPart(ref tsubpart4, 50, 559, 300, 20, 0);
      if (this.UnitNr > -1)
      {
        this.ss = "Click to change the name of the unit";
        let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BNameId = this.AddSubPart(ref tsubpart5, 370, 50, 32, 16, 1);
        let mut tsubpart6: SubPartClass =  TextPartClass::new("Name: " + this.game.Data.UnitObj[this.UnitNr].Name + " (#" + Strings.Trim(Conversion.Str( this.UnitNr)) + ")", Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 250, 20, false, tDescript: this.ss);
        this.BNameTextId = this.AddSubPart(ref tsubpart6, 410, 49, 200, 20, 0);
        this.ss = "Click to toggle if it is a unit or a HQ";
        tsubpart6 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BHQId = this.AddSubPart(ref tsubpart6, 370, 70, 32, 16, 1);
        tsubpart6 =  TextPartClass::new("HQ: " + Conversion.Str( this.game.Data.UnitObj[this.UnitNr].IsHQ), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.BHQTextId = this.AddSubPart(ref tsubpart6, 410, 69, 200, 20, 0);
        this.ss = "Click to change the coloration it is a HQ";
        tsubpart6 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a3id = this.AddSubPart(ref tsubpart6, 650, 50, 32, 16, 1);
        tsubpart6 =  TextPartClass::new("Change Coloration of Unit" + this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a3bid = this.AddSubPart(ref tsubpart6, 700, 49, 300, 20, 0);
        this.ss = "Click to make this unit identical to a unit id# to be specified";
        tsubpart6 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
        this.a4id = this.AddSubPart(ref tsubpart6, 650, 70, 32, 16, 1);
        tsubpart6 =  TextPartClass::new("Make Copy of other #", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a4bid = this.AddSubPart(ref tsubpart6, 700, 69, 300, 20, 0);
        if (this.game.Data.UnitObj[this.UnitNr].PreDef > -1)
        {
          this.ss = "Click to assign to a library";
          tsubpart6 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
          this.a11id = this.AddSubPart(ref tsubpart6, 650, 90, 32, 16, 1);
          tsubpart6 =  TextPartClass::new("Assign to Library", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.a11bid = this.AddSubPart(ref tsubpart6, 700, 89, 300, 20, 0);
        }
        this.ss = "Click to set owner of unit";
        tsubpart6 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a9id = this.AddSubPart(ref tsubpart6, 650, 130, 32, 16, 1);
        tsubpart6 =  TextPartClass::new("Owner = " + Conversion.Str( this.game.Data.UnitObj[this.UnitNr].Regime), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a9bid = this.AddSubPart(ref tsubpart6, 700, 129, 300, 20, 0);
        if (this.game.Data.UnitObj[this.UnitNr].PreDef == -1)
        {
          str1: String;
          if (this.game.Data.UnitObj[this.UnitNr].Historical == -1)
          {
            str1 = "none".to_owned();
          }
          else
          {
            let mut historical: i32 =  this.game.Data.UnitObj[this.UnitNr].Historical;
            str2: String;
            if (this.game.Data.HistoricalUnitObj[historical].Type == 1)
              str2 = "(Ind)";
            if (this.game.Data.HistoricalUnitObj[historical].Type == 2)
              str2 = "(Div)";
            if (this.game.Data.HistoricalUnitObj[historical].Type == 5)
              str2 = "(Corps)";
            if (this.game.Data.HistoricalUnitObj[historical].Type == 6)
              str2 = "(Army)";
            if (this.game.Data.HistoricalUnitObj[historical].Type == 7)
              str2 = "(ArmGrp)";
            if (this.game.Data.HistoricalUnitObj[historical].Type == 8)
              str2 = "(HighCom)";
            str1 = this.game.Data.HistoricalUnitObj[historical].Name + " " + str2;
            this.ss = "Click to set historical subpart";
            tsubpart6 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
            this.a7id = this.AddSubPart(ref tsubpart6, 650, 110, 32, 16, 1);
            tsubpart6 =  TextPartClass::new("Subpart = " + Conversion.Str( this.game.Data.UnitObj[this.UnitNr].HistoricalSubPart), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
            this.a7bid = this.AddSubPart(ref tsubpart6, 700, 109, 300, 20, 0);
          }
          if (this.game.Data.UnitObj[this.UnitNr].Historical > -1)
            this.ss = "Set as historical unit (HisNr#" + this.game.Data.UnitObj[this.UnitNr].Historical.ToString() + " ID#" + this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.UnitNr].Historical].ID.ToString() + ")";
          else
            this.ss = "Set as historical unit (" + this.game.Data.UnitObj[this.UnitNr].Historical.ToString() + ")";
          tsubpart6 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.a6id = this.AddSubPart(ref tsubpart6, 650, 90, 32, 16, 1);
          tsubpart6 =  TextPartClass::new(this.ss, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.a6bid = this.AddSubPart(ref tsubpart6, 700, 89, 300, 20, 0);
        }
        if (this.game.Data.UnitObj[this.UnitNr].PreDef > -1)
        {
          this.ss = "Click to place a copy of this predef type on the map";
          tsubpart6 =  ButtonPartClass::new(this.game.BUTTONDRAW, tDescript: this.ss);
          this.a5id = this.AddSubPart(ref tsubpart6, 10, 520, 32, 16, 1);
          tsubpart6 =  TextPartClass::new("Place this predef on map", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.a5bid = this.AddSubPart(ref tsubpart6, 50, 519, 300, 20, 0);
        }
        this.ss = "Click to set the initial ammount of supply points. In settings you can do a mass action though...";
        tsubpart6 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BSupplyId = this.AddSubPart(ref tsubpart6, 370, 90, 32, 16, 1);
        tsubpart6 =  TextPartClass::new("Supply Pts: " + Conversion.Str( this.game.Data.UnitObj[this.UnitNr].Supply), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.BSupplyTextId = this.AddSubPart(ref tsubpart6, 410, 89, 200, 20, 0);
        this.ss = "Click to remove this unit.";
        tsubpart6 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
        this.BRemoveUnitId = this.AddSubPart(ref tsubpart6, 710, 330, 32, 16, 1);
        tsubpart6 =  TextPartClass::new("Remove this Unit", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.BRemoveUnitTextId = this.AddSubPart(ref tsubpart6, 750, 329, 200, 20, 0);
        if (this.game.Data.UnitObj[this.UnitNr].SFCount > -1)
        {
          this.SFListObj = ListClass::new();
          let mut num: i32 =  -1;
          let mut sfCount: i32 =  this.game.Data.UnitObj[this.UnitNr].SFCount;
          for (let mut index: i32 =  0; index <= sfCount; index += 1)
          {
            let mut sf: i32 =  this.game.Data.UnitObj[this.UnitNr].SFList[index];
            if (this.SFNr == sf)
              num = index;
            let mut type: i32 =  this.game.Data.SFObj[sf].Type;
            this.SFListObj.add(type <= -1 ? "Empty SubFormation" : Conversion.Str( this.game.Data.SFObj[sf].Qty) + "x " + this.game.Data.SFTypeObj[type].Name, sf);
          }
          if (num == -1)
            this.SFNr = -1;
          ListClass sfListObj = this.SFListObj;
          let mut tlistselect: i32 =  num;
          let mut game: GameClass = this.game;
          ref local1: Bitmap = ref this.OwnBitmap;
          font: Font =  null;
          ref local2: Font = ref font;
          tsubpart6 =  new ListSubPartClass(sfListObj, 5, 250, tlistselect, game, tHeader: "Subformations in Unit", tbackbitmap: (ref local1), bbx: 370, bby: 120, overruleFont: (ref local2));
          this.SFListId = this.AddSubPart(ref tsubpart6, 370, 120, 250, 128, 0);
        }
        else
          this.SFNr = -1;
        this.ss = "Click to add a subformation to this unit";
        tsubpart6 =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
        this.BAddSFId = this.AddSubPart(ref tsubpart6, 370, 280, 32, 16, 1);
        tsubpart6 =  TextPartClass::new("Add SubFormation", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.BaddSFTextId = this.AddSubPart(ref tsubpart6, 410, 279, 300, 20, 0);
        if (this.SFNr > -1)
        {
          this.ss = "Click to remove selected subformation from this unit";
          tsubpart6 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
          this.BRemoveSFId = this.AddSubPart(ref tsubpart6, 370, 300, 32, 16, 1);
          tsubpart6 =  TextPartClass::new("Remove SubFormation", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.BRemoveSFTextid = this.AddSubPart(ref tsubpart6, 410, 299, 300, 20, 0);
        }
      }
      this.SpecialSFSheet();
    }

    pub fn SpecialSFSheet()
    {
      if (this.BTypeId > 0)
        this.RemoveSubPart(this.BTypeId);
      if (this.BTypeTextId > 0)
        this.RemoveSubPart(this.BTypeTextId);
      if (this.SFTypeListId > 0)
        this.RemoveSubPart(this.SFTypeListId);
      if (this.BQtyID > 0)
        this.RemoveSubPart(this.BQtyID);
      if (this.BQtyTextId > 0)
        this.RemoveSubPart(this.BQtyTextId);
      if (this.BRdnId > 0)
        this.RemoveSubPart(this.BRdnId);
      if (this.BRdnTextId > 0)
        this.RemoveSubPart(this.BRdnTextId);
      if (this.BXpId > 0)
        this.RemoveSubPart(this.BXpId);
      if (this.BXpTextId > 0)
        this.RemoveSubPart(this.BXpTextId);
      if (this.BMorId > 0)
        this.RemoveSubPart(this.BMorId);
      if (this.BMorTextId > 0)
        this.RemoveSubPart(this.BMorTextId);
      if (this.BPeopleId > 0)
        this.RemoveSubPart(this.BPeopleId);
      if (this.BPeopleTextId > 0)
        this.RemoveSubPart(this.BPeopleTextId);
      if (this.BEntrId > 0)
        this.RemoveSubPart(this.BEntrId);
      if (this.BEntrTextId > 0)
        this.RemoveSubPart(this.BEntrTextId);
      if (this.bmovid > 0)
        this.RemoveSubPart(this.bmovid);
      if (this.bmovtextid > 0)
        this.RemoveSubPart(this.bmovtextid);
      if (this.SFNr <= -1)
        return;
      let mut type: i32 =  this.game.Data.SFObj[this.SFNr].Type;
      str1: String = type <= -1 ? "Empty" : this.game.Data.SFTypeObj[type].Name;
      this.ss = "Click to select the subformationtype of this subformation";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.BTypeId = this.AddSubPart(ref tsubpart1, 710, 360, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("SFType: " + str1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.BTypeTextId = this.AddSubPart(ref tsubpart2, 750, 359, 300, 20, 0);
      this.ss = "Click to set the qty of this subformationtype in this subformation";
      str2: String = Conversion.Str( this.game.Data.SFObj[this.SFNr].Qty);
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.BQtyID = this.AddSubPart(ref tsubpart3, 710, 385, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("Qty: " + str2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.BQtyTextId = this.AddSubPart(ref tsubpart4, 750, 384, 300, 20, 0);
      this.ss = "Click to set the Readiness level of these troops";
      tsubpart4 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.BRdnId = this.AddSubPart(ref tsubpart4, 710, 405, 32, 16, 1);
      tsubpart4 =  TextPartClass::new("Rdn: " + Conversion.Str( this.game.Data.SFObj[this.SFNr].Rdn), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.BRdnTextId = this.AddSubPart(ref tsubpart4, 750, 404, 300, 20, 0);
      this.ss = "Click to set the experience level of these troops";
      tsubpart4 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.BXpId = this.AddSubPart(ref tsubpart4, 710, 430, 32, 16, 1);
      tsubpart4 =  TextPartClass::new("Xp: " + Conversion.Str( this.game.Data.SFObj[this.SFNr].Xp), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.BXpTextId = this.AddSubPart(ref tsubpart4, 750, 429, 300, 20, 0);
      this.ss = "Click to set the people these troops belong too";
      tsubpart4 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.BPeopleId = this.AddSubPart(ref tsubpart4, 410, 360, 32, 16, 1);
      tsubpart4 =  TextPartClass::new("People: " + this.game.Data.PeopleObj[this.game.Data.SFObj[this.SFNr].People].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.BPeopleTextId = this.AddSubPart(ref tsubpart4, 450, 359, 300, 20, 0);
      this.ss = "Click to set the morale of this subformation";
      tsubpart4 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.BMorId = this.AddSubPart(ref tsubpart4, 410, 385, 32, 16, 1);
      tsubpart4 =  TextPartClass::new("Morale: " + Conversion.Str( this.game.Data.SFObj[this.SFNr].Mor), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.BMorTextId = this.AddSubPart(ref tsubpart4, 450, 384, 300, 20, 0);
      this.ss = "Click to set the entrenchment of this subformation.. In settings there is a mass option that sets to auto-entrench";
      tsubpart4 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.BEntrId = this.AddSubPart(ref tsubpart4, 410, 405, 32, 16, 1);
      tsubpart4 =  TextPartClass::new("Entrench: " + Conversion.Str( this.game.Data.SFObj[this.SFNr].CurrentEntrench), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.BEntrTextId = this.AddSubPart(ref tsubpart4, 450, 404, 300, 20, 0);
      this.ss = "Click to set the movementtype overrule. -1= keep standard movement. >-1 is overrule with specific type";
      tsubpart4 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.bmovid = this.AddSubPart(ref tsubpart4, 410, 430, 32, 16, 1);
      if (this.game.Data.SFObj[this.SFNr].MoveType == -1)
      {
        tsubpart4 =  TextPartClass::new("MoveType: DEFAULT", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.bmovtextid = this.AddSubPart(ref tsubpart4, 450, 429, 300, 20, 0);
      }
      else
      {
        tsubpart4 =  TextPartClass::new("MoveType: " + this.game.Data.TempString[0 + this.game.Data.SFObj[this.SFNr].MoveType], Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.bmovtextid = this.AddSubPart(ref tsubpart4, 450, 429, 300, 20, 0);
      }
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
                this.UnitNr = -1;
                this.MakeUnitListGUI(this.UnitNr);
              }
              else if (num2 == -2)
              {
                this.LibNr = -1;
                this.UnitNr = -1;
                this.MakeUnitListGUI(this.UnitNr);
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.UnitListId)
            {
              let mut num3: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num3 > -1)
              {
                this.UnitNr = num3;
                this.MakeUnitTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BAddUnitId)
            {
              integer: i32;
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1)
              {
                this.game.ProcessingObj.NewUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, false, this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime);
              }
              else
              {
                integer = Conversions.ToInteger(Interaction.InputBox("Give Regime Nr"));
                if (integer >= -1 & integer <= this.game.Data.RegimeCounter)
                  this.game.ProcessingObj.NewUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, false, integer);
              }
              this.UnitNr = this.game.Data.UnitCounter;
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1)
                this.game.Data.UnitObj[this.UnitNr].Regime = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime;
              else
                this.game.Data.UnitObj[this.UnitNr].Regime = integer;
              this.MakeUnitListGUI(this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.UnitList2Id)
            {
              let mut num4: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num4 > -1)
              {
                this.UnitNr = num4;
                this.MakeUnitTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.bmovid)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 44, this.SFNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BAddUnit2Id)
            {
              this.UnitNr = this.game.Data.AddUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1)
              {
                this.game.Data.UnitObj[this.UnitNr].Regime = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime;
              }
              else
              {
                let mut integer: i32 =  Conversions.ToInteger(Interaction.InputBox("Give Regime Nr"));
                if (integer > -1 & integer <= this.game.Data.RegimeCounter)
                  this.game.Data.UnitObj[this.UnitNr].Regime = integer;
              }
              this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RemoveUnitFromList(this.UnitNr);
              this.game.Data.UnitObj[this.UnitNr].PreDef = this.game.HandyFunctionsObj.GetNextPreDefNr();
              this.game.Data.UnitObj[this.UnitNr].X = -1;
              this.game.Data.UnitObj[this.UnitNr].Y = -1;
              this.MakeUnitListGUI(this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.a3id)
            {
              ColorDialog colorDialog = ColorDialog::new();
              colorDialog.Color = Color.FromArgb( byte.MaxValue, this.game.Data.UnitObj[this.UnitNr].Red, this.game.Data.UnitObj[this.UnitNr].Green, this.game.Data.UnitObj[this.UnitNr].Blue);
              if (colorDialog.ShowDialog() == DialogResult.OK)
              {
                UnitClass unitClass1 = this.game.Data.UnitObj[this.UnitNr];
                color: Color = colorDialog.Color;
                let mut r: i32 =   color.R;
                unitClass1.Red = r;
                UnitClass unitClass2 = this.game.Data.UnitObj[this.UnitNr];
                color = colorDialog.Color;
                let mut g: i32 =   color.G;
                unitClass2.Green = g;
                UnitClass unitClass3 = this.game.Data.UnitObj[this.UnitNr];
                color = colorDialog.Color;
                let mut b1: i32 =   color.B;
                unitClass3.Blue = b1;
              }
              this.MakeUnitListGUI(this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.a8id)
            {
              filename: String = this.game.HandyFunctionsObj.LoadSomething("All files (*.*)|*.*", "Pick a PT file please...", this.game.AppPath, true);
              if (File.Exists(this.game.AppPath + filename))
              {
                if (Interaction.MsgBox( "Also overwrite historical units?", MsgBoxStyle.YesNo) == MsgBoxResult.Yes)
                  this.game.HandyFunctionsObj.ImportPreDefUnitsOnly(filename, true);
                else
                  this.game.HandyFunctionsObj.ImportPreDefUnitsOnly(filename);
                BitmapStore.ReloadSystemGraphics(this.game.Data.SystemGfx);
                this.game.Data.LoadGraphics((Form1) null);
                let mut num5: i32 =   Interaction.MsgBox( "Predef units (if any available) have been imported!", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                let mut num6: i32 =   Interaction.MsgBox( "Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              this.MakeUnitListGUI(this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.a4id)
            {
              str: String = Conversions.ToString(Conversion.Val(Interaction.InputBox("Give unit number# to copy from", "Shadow Empire : Planetary Conquest")));
              if (Operators.CompareString(str, "", false) != 0)
              {
                let mut funr: i32 =   Math.Round(Conversion.Val(str));
                if (funr > -1 & funr <= this.game.Data.UnitCounter)
                {
                  this.game.HandyFunctionsObj.CopyUnit(this.UnitNr, funr, false);
                }
                else
                {
                  let mut num7: i32 =   Interaction.MsgBox( "Unit # does not exist. Sorry.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
              }
              this.MakeUnitListGUI(this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.a5id)
            {
              integer: i32;
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1)
              {
                this.game.ProcessingObj.NewUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, false, this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime);
              }
              else
              {
                integer = Conversions.ToInteger(Interaction.InputBox("Give Regime Nr"));
                if (integer > -1 & integer <= this.game.Data.RegimeCounter)
                  this.game.ProcessingObj.NewUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, false, integer);
              }
              let mut unitNr: i32 =  this.UnitNr;
              this.UnitNr = this.game.Data.UnitCounter;
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1)
                this.game.Data.UnitObj[this.UnitNr].Regime = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime;
              else
                this.game.Data.UnitObj[this.UnitNr].Regime = integer;
              this.game.HandyFunctionsObj.CopyUnit(this.UnitNr, unitNr);
              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[this.UnitNr].X, this.game.Data.UnitObj[this.UnitNr].Y].Regime,  Math.Round( this.game.Data.RuleVar[99]), 99,  Math.Round( this.game.Data.RuleVar[3]), this.game.Data.UnitObj[this.UnitNr].X, this.game.Data.UnitObj[this.UnitNr].Y, 0, false, muststartonairfield: false, NoAPPenalties: true, SeaBlock: true, BlockAllSea: true);
              SimpleList simpleList = SimpleList::new();
              let mut unitCounter: i32 =  this.game.Data.UnitCounter;
              for (let mut tid: i32 =  0; tid <= unitCounter; tid += 1)
              {
                if (this.game.Data.UnitObj[tid].X > -1 && this.game.Data.UnitObj[tid].Regime == this.game.Data.UnitObj[this.UnitNr].Regime && this.game.Data.UnitObj[tid].IsHQ & tid != this.UnitNr && this.game.EditObj.TempValue[this.game.Data.UnitObj[tid].Map].Value[this.game.Data.UnitObj[tid].X, this.game.Data.UnitObj[tid].Y] < 9999)
                  simpleList.Add(tid, this.game.EditObj.TempValue[this.game.Data.UnitObj[tid].Map].Value[this.game.Data.UnitObj[tid].X, this.game.Data.UnitObj[tid].Y]);
              }
              if (simpleList.Counter > -1)
              {
                simpleList.Sort();
                let mut num8: i32 =  simpleList.Id[0];
                if (num8 > -1)
                  this.game.Data.UnitObj[this.UnitNr].HQ = num8;
              }
              this.UnitNr = unitNr;
              this.MakeUnitListGUI(this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BNameId)
            {
              str: String = Interaction.InputBox("Give new name, please.", "Give Name");
              this.game.Data.UnitObj[this.UnitNr].Name = str;
              if (this.game.Data.UnitObj[this.UnitNr].Historical > -1 && Interaction.MsgBox( "Set all of same historical unit to same name?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              {
                let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                for (let mut index2: i32 =  0; index2 <= unitCounter; index2 += 1)
                {
                  if (this.game.Data.UnitObj[index2].Historical == this.game.Data.UnitObj[this.UnitNr].Historical)
                    this.game.Data.UnitObj[index2].Name = str;
                }
              }
              this.MakeUnitListGUI(this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.a10id)
            {
              bool flag = false;
              if (Interaction.MsgBox( "Also delete predef units?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                flag = true;
              let mut num9: i32 =   Interaction.MsgBox( "Ok hold on!! this can take some while", Title: ( "Shadow Empire : Planetary Conquest"));
              this.game.FormRef.Cursor = Cursors.WaitCursor;
              for (let mut unitCounter: i32 =  this.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
              {
                if (this.game.Data.UnitObj[unitCounter].PreDef == -1 | flag)
                {
                  data: DataClass = this.game.Data;
                  let mut nr: i32 =  unitCounter;
                  let mut gameClass: GameClass = (GameClass) null;
                  ref let mut local: GameClass = ref gameClass;
                  data.RemoveUnit(nr, ref local);
                }
              }
              this.UnitNr = -1;
              this.game.FormRef.Cursor = Cursors.Default;
              let mut num10: i32 =   Interaction.MsgBox( "Done!", Title: ( "Shadow Empire : Planetary Conquest"));
              this.MakeUnitListGUI(this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BSupplyId)
            {
              let mut num11: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give new supply pts ammount, please.", "Shadow Empire : Planetary Conquest")));
              if (num11 > -1)
              {
                this.game.Data.UnitObj[this.UnitNr].Supply = num11;
              }
              else
              {
                let mut num12: i32 =   Interaction.MsgBox( "Must be positive number.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              this.MakeUnitListGUI(this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.a11id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 95, this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BRemoveUnitId)
            {
              if (this.game.Data.UnitObj[this.UnitNr].PassengerCounter > -1)
              {
                let mut num13: i32 =   Interaction.MsgBox( "No can do. Unit has passengers. remove them first.", Title: ( "Shadow Empire : Planetary Conquest"));
                return windowReturnClass;
              }
              this.game.EditObj.UnitSelected = -1;
              if (this.game.Data.UnitObj[this.UnitNr].SFCount > -1)
              {
                for (let mut sfCount: i32 =  this.game.Data.UnitObj[this.UnitNr].SFCount; sfCount >= 0; sfCount += -1)
                  this.game.Data.RemoveSF(this.game.Data.UnitObj[this.UnitNr].SFList[sfCount]);
              }
              this.game.EditObj.UnitSelected = -1;
              this.game.Data.RemoveUnit(this.UnitNr, ref this.game);
              this.UnitNr = this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter <= -1 ? -1 : this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].UnitList[0];
              this.SFNr = -1;
              this.MakeUnitListGUI(this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.SFListId)
            {
              let mut num14: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num14 > -1)
              {
                this.SFNr = num14;
                this.MakeUnitTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BAddSFId)
            {
              this.SFNr = this.game.Data.AddSF(this.UnitNr);
              this.game.Data.SFObj[this.SFNr].Type = 0;
              this.game.Data.SFObj[this.SFNr].Rdn = 100;
              this.game.Data.SFObj[this.SFNr].Xp = 0;
              if (this.game.Data.UnitObj[this.UnitNr].Regime > -1)
              {
                this.game.Data.SFObj[this.SFNr].People = this.game.Data.RegimeObj[this.game.Data.UnitObj[this.UnitNr].Regime].People;
                this.game.Data.SFObj[this.SFNr].Mor = this.game.Data.PeopleObj[this.game.Data.SFObj[this.SFNr].People].BaseMorale[this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.UnitObj[this.UnitNr].Regime].People].PeopleGroup];
              }
              else
              {
                this.game.Data.SFObj[this.SFNr].People = 0;
                this.game.Data.SFObj[this.SFNr].Mor = this.game.Data.PeopleObj[this.game.Data.SFObj[this.SFNr].People].BaseMorale[0];
              }
              this.MakeUnitTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BRemoveSFId)
            {
              this.game.Data.RemoveSF(this.SFNr);
              this.SFNr = -1;
              this.MakeUnitTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.SFTypeListId)
            {
              let mut num15: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num15 > -1)
              {
                this.SFTypeNr = num15;
                this.SpecialSFSheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BQtyID)
            {
              let mut num16: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Qty for this Subformation:", "Shadow Empire : Planetary Conquest")));
              if (num16 < 1 | num16 > 9999)
              {
                let mut num17: i32 =   Interaction.MsgBox( "Cancelled. You should have entered a value between 1 and 9999.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                this.game.Data.SFObj[this.SFNr].Qty = num16;
                this.MakeUnitTypeItemGUI();
                windowReturnClass.SetFlag(true);
              }
              return windowReturnClass;
            }
            if (num1 == this.BRdnId)
            {
              let mut num18: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Rdn for this Subformation:", "Shadow Empire : Planetary Conquest")));
              if (num18 < 10 | num18 > 100)
              {
                let mut num19: i32 =   Interaction.MsgBox( "Cancelled. You should have entered a value between 10 and 100.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                this.game.Data.SFObj[this.SFNr].Rdn = num18;
                this.MakeUnitTypeItemGUI();
                windowReturnClass.SetFlag(true);
              }
              return windowReturnClass;
            }
            if (num1 == this.BXpId)
            {
              let mut num20: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Xp for this Subformation:", "Shadow Empire : Planetary Conquest")));
              if (num20 < 0 | num20 > 300)
              {
                let mut num21: i32 =   Interaction.MsgBox( "Cancelled. You should have entered a value between 10 and 300.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                this.game.Data.SFObj[this.SFNr].Xp = num20;
                this.MakeUnitTypeItemGUI();
                windowReturnClass.SetFlag(true);
              }
              return windowReturnClass;
            }
            if (num1 == this.BMorId)
            {
              let mut num22: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Morale for this Subformation:", "Shadow Empire : Planetary Conquest")));
              if (num22 < 1 | num22 > 100)
              {
                let mut num23: i32 =   Interaction.MsgBox( "Cancelled. You should have entered a value between 1 and 100.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                this.game.Data.SFObj[this.SFNr].Mor = num22;
                this.MakeUnitTypeItemGUI();
                windowReturnClass.SetFlag(true);
              }
              return windowReturnClass;
            }
            if (num1 == this.a9id)
            {
              let mut num24: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Regime #", "Shadow Empire : Planetary Conquest")));
              if (num24 < -1 | num24 > this.game.Data.RegimeCounter)
              {
                let mut num25: i32 =   Interaction.MsgBox( "Cancelled. Not a valid regime.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                this.game.Data.UnitObj[this.UnitNr].Regime = num24;
                this.MakeUnitListGUI(this.UnitNr);
                windowReturnClass.SetFlag(true);
              }
              return windowReturnClass;
            }
            if (num1 == this.BEntrId)
            {
              let mut num26: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Entrench for this Subformation:", "Shadow Empire : Planetary Conquest")));
              if (num26 < 0 | num26 > 999)
              {
                let mut num27: i32 =   Interaction.MsgBox( "Cancelled. You should have entered a value between 1 and 999.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                this.game.Data.SFObj[this.SFNr].CurrentEntrench = num26;
                this.MakeUnitTypeItemGUI();
                windowReturnClass.SetFlag(true);
              }
              return windowReturnClass;
            }
            if (num1 == this.a6id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 37, this.UnitNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.a7id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 48, this.UnitNr, this.game.Data.UnitObj[this.UnitNr].Historical);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BPeopleId)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 7, this.SFNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BTypeId)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 6, this.SFNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BHQId)
            {
              this.game.Data.UnitObj[this.UnitNr].IsHQ = !this.game.Data.UnitObj[this.UnitNr].IsHQ;
              this.MakeUnitListGUI(this.UnitNr);
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
