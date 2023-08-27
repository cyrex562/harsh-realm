// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.BuildWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class BuildWindowClass : WindowClass
  {
     int LocNr;
     int BNameId;
     int BNameTextId;
     int B1Id;
     int B1TextId;
     int B2Id;
     int B2TextId;
     int B3Id;
     int B3TextId;
     int[] VarId;
     int Text1Id;
     int Text2Id;
     int Text3Id;
     int OptionsListId;
     ATListClass OptionsListObj;
     int OptionsList2Id;
     ATListClass OptionsList2Obj;
     int detailnr;
     bool[] LocCanConstr;
     bool[] LocCanConstr2;
     bool LocCanSetup;

    pub BuildWindowClass(ref tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base(ref tGame, 1024, 200, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.VarId = new int[5];
      this.LocCanConstr = new bool[1000];
      this.LocCanConstr2 = new bool[1000];
      this.LocNr = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
      this.detailnr = -1;
      this.game.EditObj.LocTypeSelected = -1;
      this.dostuff();
      this.fixshade = true;
    }

     void dostuff()
    {
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
        this.RemoveSubPart(this.Text2Id);
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.B1Id > 0)
        this.RemoveSubPart(this.B1Id);
      if (this.B1TextId > 0)
        this.RemoveSubPart(this.B1TextId);
      if (this.B2Id > 0)
        this.RemoveSubPart(this.B2Id);
      if (this.B2TextId > 0)
        this.RemoveSubPart(this.B2TextId);
      let mut index1: i32 =  0;
      do
      {
        if (this.VarId[index1] > 0)
          this.RemoveSubPart(this.VarId[index1]);
        index1 += 1;
      }
      while (index1 <= 4);
      this.NewBackGroundAndClearAll(1024, 200, -1);
      Graphics objGraphics = Graphics.FromImage((Image) this.OwnBitmap);
      ref Graphics local1 = ref objGraphics;
      Rectangle rectangle1 = Rectangle::new(0, 0, 350, 14);
      let mut rect1_1: &Rectangle = &rectangle1
      Rectangle rectangle2;
      let mut rect2_1: &Rectangle = &rectangle2
      DrawMod.MakeFullBoxVic2(ref local1, rect1_1, "LOCATION TYPE                         EP                   PP                 SUP", rect2_1, "");
      if (!this.LocCanSetup)
      {
        this.game.FormRef.Cursor = Cursors.WaitCursor;
        if (this.game.Data.LocTypeCounter > -1)
        {
          let mut locTypeCounter: i32 =  this.game.Data.LocTypeCounter;
          for (let mut loctype: i32 =  0; loctype <= locTypeCounter; loctype += 1)
          {
            if (this.game.EditObj.OrderUnit > -1)
            {
              if (this.game.HandyFunctionsObj.CanConstructLoc(loctype, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, this.game.Data.Turn, this.game.EditObj.UnitSelected, true))
                this.LocCanConstr[loctype] = true;
              if (this.game.HandyFunctionsObj.CanConstructLoc(loctype, this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.Data.Turn, this.game.EditObj.OrderUnit))
                this.LocCanConstr2[loctype] = true;
            }
          }
        }
        this.game.FormRef.Cursor = Cursors.Default;
        this.LocCanSetup = true;
      }
      int tlistselect;
      str1: String;
      SubPartClass tsubpart;
      if (this.OptionsListId == 0)
      {
        this.OptionsListObj = ATListClass::new();
        if (this.detailnr > this.game.Data.LocTypeCounter)
          this.detailnr = -1;
        tlistselect = -1;
        let mut num: i32 =  -1;
        str1 = "NAME".to_owned();
        if (this.game.Data.LocTypeCounter > -1)
        {
          let mut locTypeCounter: i32 =  this.game.Data.LocTypeCounter;
          for (let mut tdata: i32 =  0; tdata <= locTypeCounter; tdata += 1)
          {
            bool flag = this.LocCanConstr[tdata];
            if (this.game.Data.LocTypeObj[tdata].HumanCanBuild & this.game.Data.LocTypeObj[tdata].Buildable & this.game.Data.LocTypeObj[tdata].EPCost == 0 & this.game.Data.LocTypeObj[tdata].SupplyCost == 0)
              flag = true;
            if (flag)
            {
              num += 1;
              if (this.detailnr == -1)
                this.detailnr = tdata;
              if (this.detailnr == tdata)
                tlistselect = num;
              str2: String = this.game.Data.LocTypeObj[tdata].Name;
              if (Strings.Len(str2) > 22)
                str2 = Strings.Left(str2, 22);
              this.OptionsListObj.add(str2, tdata, Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[tdata].EPCost)), Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[tdata].PPCost)), Strings.Trim(Conversion.Str(  Math.Round( ( this.game.Data.LocTypeObj[tdata].SupplyCost / this.game.Data.RuleVar[77])))));
            }
          }
          if (this.OptionsListId > 0)
          {
            this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, tlistselect);
            this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
          }
          else
          {
            tsubpart =  new ATListSubPartClass(this.OptionsListObj, 10, 350, tlistselect, this.game, true, tShowPair: true, tValueWidth: 200, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 0, bby: 16);
            this.OptionsListId = this.AddSubPart(ref tsubpart, 0, 16, 350, 176, 0);
          }
        }
      }
      if (this.detailnr > -1)
      {
        DrawMod.DrawRectangle(ref objGraphics, 399, 44, 252, 105,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
        str1 = this.game.Data.LocTypeObj[this.detailnr].Name;
        if (this.game.Data.LocTypeObj[this.detailnr].OverdrawLTNr > -1)
        {
          ref Graphics local2 = ref objGraphics;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.detailnr].OverdrawLTNr].BasicPicID[this.game.Data.LocTypeObj[this.detailnr].OverdrawSpriteNr]);
          ref local3: Bitmap = ref bitmap;
          DrawMod.DrawScaled(ref local2, ref local3, 400, 45, 250, 103);
        }
        else if (this.game.Data.LocTypeObj[this.detailnr].PictureLT > -1)
        {
          ref Graphics local4 = ref objGraphics;
          bitmap1: Bitmap = BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].LandscapeType].BasicPicID[this.game.Data.MapObj[0].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].SpriteNr]);
          ref local5: Bitmap = ref bitmap1;
          DrawMod.DrawScaled(ref local4, ref local5, 400, 45, 250, 103);
          ref Graphics local6 = ref objGraphics;
          bitmap2: Bitmap = BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.detailnr].PictureLT].BasicPicID[this.game.Data.LocTypeObj[this.detailnr].PictureSprite]);
          ref local7: Bitmap = ref bitmap2;
          DrawMod.DrawScaled(ref local6, ref local7, 400, 45, 250, 103);
        }
        else
        {
          ref Graphics local8 = ref objGraphics;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].LandscapeType].BasicPicID[this.game.Data.MapObj[0].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].SpriteNr]);
          ref local9: Bitmap = ref bitmap;
          DrawMod.DrawScaled(ref local8, ref local9, 400, 45, 250, 103);
        }
        ref Graphics local10 = ref objGraphics;
        rectangle1 = Rectangle::new(399, 0, 250, 14);
        let mut rect1_2: &Rectangle = &rectangle1
        Rectangle rectangle3 = Rectangle::new(399, 14, 250, 23);
        let mut rect2_2: &Rectangle = &rectangle3
        name: String = this.game.Data.LocTypeObj[this.detailnr].Name;
        DrawMod.MakeFullBoxVic2(ref local10, rect1_2, "SELECTED LOCATION TYPE", rect2_2, name);
        str3: String = this.game.Data.LocTypeObj[this.detailnr].SetPeopleToSlotX <= 0 ? this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.Turn].People].Name : this.game.Data.PeopleObj[this.game.Data.MapObj[0].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].AreaCode[this.game.Data.LocTypeObj[this.detailnr].SetPeopleToSlotX]].Name;
        if (this.game.Data.LocTypeObj[this.detailnr].UpgradeNr > -1)
          str3 = this.game.Data.PeopleObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].Location].People].Name;
        ref Graphics local11 = ref objGraphics;
        rectangle3 = Rectangle::new(700, 0, 130, 14);
        let mut rect1_3: &Rectangle = &rectangle3
        rectangle1 = Rectangle::new(700, 14, 130, 23);
        let mut rect2_3: &Rectangle = &rectangle1
        txt2: String = str3;
        DrawMod.MakeFullBoxVic2(ref local11, rect1_3, "LOC PEOPLE", rect2_3, txt2);
        this.OptionsList2Obj = ATListClass::new();
        str4: String = "0".to_owned();
        if (this.game.EditObj.OrderUnit > -1)
          str4 = Strings.Trim(Conversion.Str( this.game.HandyFunctionsObj.GetUnitEP(this.game.EditObj.OrderUnit)));
        str5: String = Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.detailnr].EPCost));
        tvalue3_1: String = Conversion.Val(str4) < Conversion.Val(str5) ? "SHORT" : "OK".to_owned();
        this.OptionsList2Obj.add("EP", -1, str5, str4, tvalue3_1);
        str6: String = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[this.game.Data.Turn].ResPts));
        str7: String = Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.detailnr].PPCost));
        tvalue3_2: String = Conversion.Val(str6) < Conversion.Val(str7) ? "SHORT" : "OK".to_owned();
        this.OptionsList2Obj.add("PP", -1, str7, str6, tvalue3_2);
        str8: String = "0".to_owned();
        if (this.game.EditObj.OrderUnit > -1)
          str8 = Strings.Trim(Conversion.Str( this.game.HandyFunctionsObj.HQSupplyChain(this.game.EditObj.OrderUnit, true)));
        str9: String = Strings.Trim(Conversion.Str(  Math.Round( ( this.game.Data.LocTypeObj[this.detailnr].SupplyCost / this.game.Data.RuleVar[77]))));
        tvalue3_3: String = Conversion.Val(str8) < Conversion.Val(str9) ? "SHORT" : "OK".to_owned();
        this.OptionsList2Obj.add("SUP", -1, str9, str8, tvalue3_3);
        let mut index2: i32 =  0;
        do
        {
          if (this.game.Data.LocTypeObj[this.detailnr].VarType[index2] > -1)
          {
            str10: String = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[this.game.Data.LocTypeObj[this.detailnr].VarType[index2]]));
            str11: String = Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.detailnr].VarQty[index2]));
            tvalue3_4: String = Conversion.Val(str10) < Conversion.Val(str11) ? "SHORT" : "OK".to_owned();
            this.OptionsList2Obj.add(this.game.Data.RegimeSlotName[this.game.Data.LocTypeObj[this.detailnr].VarType[index2]], -1, str11, str10, tvalue3_4);
          }
          index2 += 1;
        }
        while (index2 <= 4);
        ref Graphics local12 = ref objGraphics;
        rectangle3 = Rectangle::new(700, 50, 310, 14);
        let mut rect1_4: &Rectangle = &rectangle3
        let mut rect2_4: &Rectangle = &rectangle2
        DrawMod.MakeFullBoxVic2(ref local12, rect1_4, "COST TYPE          NEED               AVAILABLE      STATUS", rect2_4, "");
        if (this.OptionsList2Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList2Id)].Refresh(this.OptionsList2Obj, tlistselect);
          this.SubPartFlag[this.SubpartNr(this.OptionsList2Id)] = true;
        }
        else
        {
          tsubpart =  new ATListSubPartClass(this.OptionsList2Obj, 6, 310, tlistselect, this.game, true, tHighlight: false, tShowPair: true, tValueWidth: 225, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 700, bby: 66);
          this.OptionsList2Id = this.AddSubPart(ref tsubpart, 700, 66, 300, 112, 0);
        }
        tsubpart =  new TextButtonPartClass("?", 40, "Get more information on selected location type", ref this.OwnBitmap, 400, 157);
        this.B1Id = this.AddSubPart(ref tsubpart, 400, 157, 40, 35, 1);
        if (this.LocCanConstr2[this.detailnr])
        {
          tsubpart =  new TextButtonPartClass("Build", 196, "Build the selected location on this hex", ref this.OwnBitmap, 450, 157);
          this.B2Id = this.AddSubPart(ref tsubpart, 450, 157, 196, 35, 1);
        }
        else
        {
          tsubpart =  new TextButtonPartClass("Build", 196, "You cannot build this location type", ref this.OwnBitmap, 450, 157, true);
          this.Text2Id = this.AddSubPart(ref tsubpart, 450, 157, 196, 35, 1);
        }
      }
      if (Information.IsNothing( objGraphics))
        return;
      objGraphics.Dispose();
    }

    pub fn PopUpRefresh() => this.DoRefresh();

    pub HandleMouseClick: WindowReturnClass(int x, int y, int b)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num1: i32 =  this.SubPartID[index];
            if (num1 == this.B1Id)
            {
              this.game.EditObj.LocTypeSelected = this.detailnr;
              this.game.EditObj.PopupValue = 12;
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.AddCommand(5, 10);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B2Id)
            {
              if (this.detailnr > -1)
              {
                if (this.game.Data.LocTypeObj[this.detailnr].HumanCanBuild)
                  this.game.EditObj.OrderUnit = -1;
                OrderResult orderResult = this.game.ProcessingObj.Build(this.game.EditObj.OrderUnit, this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.detailnr);
                if (this.game.EditObj.SoundOn)
                  SoundMod.PlayAWave(this.game.AppPath + "sound/building.wav", ref this.game.EditObj);
                if (orderResult.OK)
                {
                  this.game.EditObj.OrderType = 0;
                  this.game.ProcessingObj.LocationProductionPrognosis();
                  windowReturnClass.AddCommand(1, 5);
                  windowReturnClass.AddCommand(2, 20);
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 18);
                  windowReturnClass.AddCommand(4, 66);
                  windowReturnClass.AddCommand(4, 44);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
            }
            else if (num1 == this.OptionsListId)
            {
              let mut num2: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num2 > -1)
              {
                this.detailnr = num2;
                this.dostuff();
              }
              if (num2 == -2)
              {
                this.detailnr = -1;
                this.dostuff();
              }
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
