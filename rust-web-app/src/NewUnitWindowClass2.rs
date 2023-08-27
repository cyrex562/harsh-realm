// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.NewUnitWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class NewUnitWindowClass2 : WindowClass
  {
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     B6Id: i32;
     B6TextId: i32;
     off1id: i32;
     detailnr: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     Pic1Id: i32;
     detailnr2: i32;
     OrderTextId: i32;
     OrderText2Id: i32;
     OrderUpId: i32;
     OrderDownId: i32;
     ExtraId: i32;
     steppy: i32;
     typpy: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
     bool Hq;
     w: i32;
     h: i32;
     int[] Ucnt;
     int[] ModCnt;
     ModSubCnt: Vec<i32>;
     SubCnt: Vec<i32>;
     int[] modelcount;
     bool[] creatable;
     errors: Vec<String>;

    pub NewUnitWindowClass2( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base( tGame, tGame.ScreenWidth, 222, BackSprite: tGame.MARCBOTBAR)
    {
      this.Ucnt = new int[1];
      this.ModCnt = new int[1];
      this.ModSubCnt = new int[1, 1];
      this.SubCnt = new int[1, 1];
      this.modelcount = new int[1];
      this.creatable = new bool[1];
      this.errors = new string[1];
      this.w = tGame.ScreenWidth;
      this.h = 222;
      this.BlockBlit = true;
      this.detailnr = -1;
      this.typpy = -1;
      this.detailnr = -1;
      this.detailnr2 = -1;
      if (this.game.Data.Round == 0 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].Regime > -1)
        this.game.Data.Turn = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].Regime;
      this.calc();
      this.dostuff();
    }

     void calc()
    {
      this.Ucnt = new int[this.game.Data.HistoricalUnitCounter + 1];
      this.ModCnt = new int[this.game.Data.HistoricalUnitCounter + 1];
      this.SubCnt = new int[this.game.Data.HistoricalUnitCounter + 1, 10];
      this.ModSubCnt = new int[this.game.Data.HistoricalUnitCounter + 1, 10];
      this.modelcount = new int[this.game.Data.HistoricalUnitCounter + 1];
      this.creatable = new bool[this.game.Data.HistoricalUnitCounter + 1];
      this.errors = new string[this.game.Data.HistoricalUnitCounter + 1];
      let mut unitCounter: i32 =  this.game.Data.UnitCounter;
      for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
      {
        if (this.game.Data.UnitObj[index1].Historical > -1 & this.game.Data.UnitObj[index1].PreDef == -1 & this.game.Data.UnitObj[index1].Regime == this.game.Data.Turn)
        {
          int[] ucnt = this.Ucnt;
          int[] numArray1 = ucnt;
          let mut historical1: i32 =  this.game.Data.UnitObj[index1].Historical;
          let mut index2: i32 =  historical1;
          let mut num1: i32 =  ucnt[historical1] + 1;
          numArray1[index2] = num1;
          if (this.game.Data.UnitObj[index1].HistoricalSubPart > -1)
          {
            subCnt: Vec<i32> = this.SubCnt;
            numArray2: Vec<i32> = subCnt;
            let mut historical2: i32 =  this.game.Data.UnitObj[index1].Historical;
            let mut index3: i32 =  historical2;
            let mut historicalSubPart: i32 =  this.game.Data.UnitObj[index1].HistoricalSubPart;
            let mut index4: i32 =  historicalSubPart;
            let mut num2: i32 =  subCnt[historical2, historicalSubPart] + 1;
            numArray2[index3, index4] = num2;
          }
        }
      }
      let mut historicalUnitCounter1: i32 =  this.game.Data.HistoricalUnitCounter;
      for (let mut index5: i32 =  0; index5 <= historicalUnitCounter1; index5 += 1)
      {
        if (this.game.Data.HistoricalUnitObj[index5].ModelMaster > -1 && this.Ucnt[index5] > 0)
        {
          int[] modelcount = this.modelcount;
          int[] numArray = modelcount;
          let mut modelMaster: i32 =  this.game.Data.HistoricalUnitObj[index5].ModelMaster;
          let mut index6: i32 =  modelMaster;
          let mut num: i32 =  modelcount[modelMaster] + 1;
          numArray[index6] = num;
        }
      }
      let mut historicalUnitCounter2: i32 =  this.game.Data.HistoricalUnitCounter;
      for (let mut hisnr: i32 =  0; hisnr <= historicalUnitCounter2; hisnr += 1)
      {
        this.errors[hisnr] = "";
        if (this.game.Data.HistoricalUnitObj[hisnr].Model)
        {
          this.creatable[hisnr] = false;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].ResPts < this.game.Data.HistoricalUnitObj[hisnr].PP)
            this.errors[hisnr] = "Not enough PP to create.";
          else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter + this.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(hisnr) >= 16)
            this.errors[hisnr] = "To many units in hex to add new units.";
          else if (this.modelcount[hisnr] >= this.game.Data.HistoricalUnitObj[hisnr].MaxPresent & this.game.Data.HistoricalUnitObj[hisnr].MaxPresent != -1)
            this.errors[hisnr] = "Maximum ammount of this model already on map.";
          else
            this.creatable[hisnr] = true;
        }
      }
      let mut historicalUnitCounter3: i32 =  this.game.Data.HistoricalUnitCounter;
      for (let mut index7: i32 =  0; index7 <= historicalUnitCounter3; index7 += 1)
      {
        let mut index8: i32 =  0;
        do
        {
          if (this.game.Data.HistoricalUnitObj[index7].SubParts[index8] > -1)
          {
            int[] modCnt = this.ModCnt;
            int[] numArray = modCnt;
            let mut index9: i32 =  index7;
            let mut index10: i32 =  index9;
            let mut num: i32 =  modCnt[index9] + 1;
            numArray[index10] = num;
          }
          modSubCnt: Vec<i32> = this.ModSubCnt;
          numArray3: Vec<i32> = modSubCnt;
          let mut index11: i32 =  index7;
          let mut index12: i32 =  index11;
          let mut index13: i32 =  index8;
          let mut index14: i32 =  index13;
          let mut num3: i32 =  modSubCnt[index11, index13] + 1;
          numArray3[index12, index14] = num3;
          index8 += 1;
        }
        while (index8 <= 9);
      }
    }

     void dostuff()
    {
      if (this.off1id > 0)
        this.RemoveSubPart(this.off1id);
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
        this.RemoveSubPart(this.Text2Id);
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.Pic1Id > 0)
        this.RemoveSubPart(this.Pic1Id);
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
      if (this.B5Id > 0)
        this.RemoveSubPart(this.B5Id);
      if (this.B5TextId > 0)
        this.RemoveSubPart(this.B5TextId);
      if (this.B6Id > 0)
        this.RemoveSubPart(this.B6Id);
      if (this.B6TextId > 0)
        this.RemoveSubPart(this.B6TextId);
      if (this.OrderUpId > 0)
        this.RemoveSubPart(this.OrderUpId);
      if (this.OrderDownId > 0)
        this.RemoveSubPart(this.OrderDownId);
      if (this.ExtraId > 0)
        this.RemoveSubPart(this.ExtraId);
      if (this.OrderTextId > 0)
        this.RemoveSubPart(this.OrderTextId);
      if (this.OrderText2Id > 0)
        this.RemoveSubPart(this.OrderText2Id);
      if (this.OptionsList2Id > 0)
      {
        this.RemoveSubPart(this.OptionsList2Id);
        this.OptionsList2Id = 0;
      }
      let mut num1: i32 =   Math.Round( (this.w - 1024) / 2.0);
      this.NewBackGroundAndClearAll(this.w, this.h, this.game.MARCBOTBAR);
      this.ClearMouse();
      Graphics objgraphics = Graphics.FromImage((Image) this.OwnBitmap);
      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].UnitCounter < 15)
      {
        this.OptionsListObj = ListClass::new();
        let mut num2: i32 =  -1;
        let mut tlistselect1: i32 =  -1;
        let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
        for (let mut tdata: i32 =  0; tdata <= historicalUnitCounter; tdata += 1)
        {
          if ((uint) Strings.InStr(this.game.Data.HistoricalUnitObj[tdata].Name, "SP") > 0U)
            tdata = tdata;
          if (this.game.Data.HistoricalUnitObj[tdata].Model & this.game.Data.HistoricalUnitObj[tdata].TempRegime == this.game.Data.Turn && this.game.Data.HistoricalUnitObj[tdata].MaxPresent != 0)
          {
            num2 += 1;
            if (this.detailnr == -1)
              this.detailnr = tdata;
            if (tdata == this.detailnr)
              tlistselect1 = num2;
            tvalue4: String = !this.creatable[tdata] ? "-" : "OK".to_owned();
            tvalue3: String = "Unl.";
            if (this.game.Data.HistoricalUnitObj[tdata].MaxPresent > -1)
              tvalue3 = Conversion.Str( this.game.Data.HistoricalUnitObj[tdata].MaxPresent);
            this.OptionsListObj.add(this.game.Data.HistoricalUnitObj[tdata].Name, tdata, Conversion.Str( this.game.Data.HistoricalUnitObj[tdata].PP), Conversion.Str( this.modelcount[tdata]), tvalue3, tvalue4);
          }
        }
        if (this.OptionsListId > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, tlistselect1);
          this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
        }
        else
        {
          ListClass optionsListObj = this.OptionsListObj;
          let mut tlistselect2: i32 =  tlistselect1;
          let mut game: GameClass = this.game;
           local1: Bitmap =  this.OwnBitmap;
          let mut bbx: i32 =  num1 + 10;
          font: Font =  null;
           local2: Font =  font;
          let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsListObj, 10, 600, tlistselect2, game, tShowPair: true, tValueWidth: 350, tdotopandbottom: false, tbackbitmap: ( local1), bbx: bbx, bby: 30, tMarcStyle: true, overruleFont: ( local2));
          this.OptionsListId = this.AddSubPart( tsubpart, num1 + 10, 30, 600, 176, 0);
        }
        DrawMod.DrawTextColouredMarc( objgraphics, "UNIT MODEL", this.game.MarcFont5, num1 + 20, 13, Color.White);
        DrawMod.DrawTextColouredMarc( objgraphics, "POL.PTS", this.game.MarcFont5, num1 + 245, 13, Color.White);
        DrawMod.DrawTextColouredMarc( objgraphics, "ON MAP", this.game.MarcFont5, num1 + 331, 13, Color.White);
        DrawMod.DrawTextColouredMarc( objgraphics, "MAX ON MAP", this.game.MarcFont5, num1 + 419, 13, Color.White);
        DrawMod.DrawTextColouredMarc( objgraphics, "CAN CREATE?", this.game.MarcFont5, num1 + 505, 13, Color.White);
        if (this.detailnr > -1)
        {
          if (this.creatable[this.detailnr])
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("CREATE UNIT", 180, "Click to create this unit [SPACE]",  this.BackBitmap, num1 + 730, 80, theight: 50, usefont: this.game.MarcFont1, useshadow: true, tMarcStyle: true);
            this.B3Id = this.AddSubPart( tsubpart, num1 + 730, 80, 180, 50, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("CREATE UNIT", 180, this.errors[this.detailnr],  this.BackBitmap, num1 + 730, 80, true, theight: 50, usefont: this.game.MarcFont1, useshadow: true, tMarcStyle: true);
            this.B4Id = this.AddSubPart( tsubpart, num1 + 730, 80, 180, 50, 0);
          }
        }
      }
      if (Information.IsNothing( objgraphics))
        return;
      objgraphics.Dispose();
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult1 = OrderResult::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.B3Id)
            {
              this.game.ProcessingObj.AddNewUnitBasedOnHistorical(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.Data.Turn, this.detailnr);
              this.game.EditObj.OrderType = 0;
              this.game.EditObj.OrderUnit = -1;
              this.game.EditObj.UnitSelected = this.game.Data.UnitCounter;
              windowReturnClass.AddCommand(1, 5);
              windowReturnClass.AddCommand(2, 69);
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B1Id)
            {
              OrderResult orderResult2 = this.game.ProcessingObj.NewUnit(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.MapSelected, false, this.game.Data.Turn);
              if (this.game.EditObj.SoundOn)
                SoundMod.PlayAWave(this.game.AppPath + "sound/radio.wav",  this.game.EditObj);
              if (orderResult2.OK)
              {
                let mut num2: i32 =  0;
                let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                for (let mut index2: i32 =  0; index2 <= unitCounter; index2 += 1)
                {
                  if (this.game.Data.UnitObj[index2].IsHQ & this.game.Data.UnitObj[index2].PreDef == -1 & this.game.Data.UnitObj[index2].X > -1 && this.game.Data.UnitObj[index2].Regime == this.game.Data.Turn)
                    num2 = 1;
                }
                if (num2 == 1)
                {
                  this.game.EditObj.OrderType = 3;
                  this.game.EditObj.OrderUnit = this.game.Data.UnitCounter;
                  this.game.EditObj.UnitSelected = this.game.Data.UnitCounter;
                  windowReturnClass.AddCommand(1, 5);
                  windowReturnClass.AddCommand(2, 69);
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                this.game.EditObj.OrderType = 0;
                this.game.EditObj.UnitSelected = this.game.Data.UnitCounter;
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 69);
                this.game.EditObj.TempCoordList = CoordList::new();
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == this.OptionsListId)
              {
                let mut num3: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num3 > -1)
                {
                  this.detailnr = num3;
                  this.dostuff();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.OptionsList2Id)
              {
                let mut num4: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num4 > -1)
                {
                  this.detailnr2 = num4;
                  this.dostuff();
                }
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
