// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ProdWindowClass
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
  pub class ProdWindowClass : WindowClass
  {
     LocNr: i32;
     BNameId: i32;
     BNameTextId: i32;
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     OptionsListId: i32;
     ATListClass OptionsListObj;
     OptionsList2Id: i32;
     ATListClass OptionsList2Obj;
     detailnr: i32;
     detailnr2: i32;
     slotty: i32;
     int[] SlotButId;
     int[] SlotBut2Id;
     int[] SlotText1;
     int[] SlotText2;
     int[] SlotText3;
     int[] SlotText4;
     int[] SlotText5;
     int[] SlotSlider;
     PercentBut: Vec<i32>;
     int[] PercentX;
     totpercent: i32;
     Qball: i32;
     FlapBall: i32;
     HqBall: i32;
     CurrentHQ: i32;

    pub ProdWindowClass(
       tGame: GameClass,
      screenbitmap: Bitmap = null,
      let mut sx: i32 = -1,
      let mut sy: i32 = -1,
      let mut tl1: i32 = -1,
      let mut tl2: i32 = -1,
      let mut tl3: i32 = -1)
      : base( tGame, 1024, 200, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.SlotButId = new int[4];
      this.SlotBut2Id = new int[4];
      this.SlotText1 = new int[4];
      this.SlotText2 = new int[4];
      this.SlotText3 = new int[4];
      this.SlotText4 = new int[4];
      this.SlotText5 = new int[4];
      this.SlotSlider = new int[4];
      this.PercentBut = new int[4, 11];
      this.PercentX = new int[4];
      this.LocNr = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
      this.detailnr = -1;
      this.detailnr2 = -1;
      this.slotty = -1;
      if (this.game.Data.LocObj[this.LocNr].Production[0] > -1)
      {
        this.slotty = 0;
        this.detailnr = this.game.Data.LocObj[this.LocNr].Production[0];
        this.detailnr2 = this.game.Data.ItemTypeObj[this.detailnr].ItemGroup;
      }
      if (tl1 > -1 | tl2 > -1)
      {
        this.detailnr = tl1;
        this.detailnr2 = tl2;
        this.slotty = tl3;
      }
      this.dostuff();
    }

    pub fn PopUpRefresh() => this.DoRefresh();

    pub fn DoRefresh()
    {
      if (this.game.EditObj.OrderLoc < 0)
        return;
      this.LocNr = this.game.EditObj.OrderLoc;
      this.detailnr = -1;
      this.detailnr2 = 0;
      this.slotty = 0;
      if (this.LocNr > -1 && this.game.Data.LocObj[this.LocNr].Production[0] > -1)
      {
        this.slotty = 0;
        this.detailnr = this.game.Data.LocObj[this.LocNr].Production[0];
        this.detailnr2 = this.game.Data.ItemTypeObj[this.detailnr].ItemGroup;
      }
      this.dostuff();
    }

     void dostuff()
    {
      if (this.Qball > 0)
        this.RemoveSubPart(this.Qball);
      if (this.HqBall > 0)
        this.RemoveSubPart(this.HqBall);
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
      if (this.B3Id > 0)
        this.RemoveSubPart(this.B3Id);
      if (this.FlapBall > 0)
        this.RemoveSubPart(this.FlapBall);
      if (this.B3TextId > 0)
        this.RemoveSubPart(this.B3TextId);
      this.totpercent = 0;
      let mut index1: i32 = 0;
      do
      {
        if (this.SlotButId[index1] > 0)
          this.RemoveSubPart(this.SlotButId[index1]);
        if (this.SlotBut2Id[index1] > 0)
          this.RemoveSubPart(this.SlotBut2Id[index1]);
        if (this.SlotText1[index1] > 0)
          this.RemoveSubPart(this.SlotText1[index1]);
        if (this.SlotText2[index1] > 0)
          this.RemoveSubPart(this.SlotText2[index1]);
        if (this.SlotText3[index1] > 0)
          this.RemoveSubPart(this.SlotText3[index1]);
        if (this.SlotText4[index1] > 0)
          this.RemoveSubPart(this.SlotText4[index1]);
        if (this.SlotText5[index1] > 0)
          this.RemoveSubPart(this.SlotText5[index1]);
        if (this.SlotSlider[index1] > 0)
          this.RemoveSubPart(this.SlotSlider[index1]);
        let mut index2: i32 = 0;
        do
        {
          if (this.PercentBut[index1, index2] > 0)
            this.RemoveSubPart(this.PercentBut[index1, index2]);
          index2 += 1;
        }
        while (index2 <= 10);
        if (this.PercentX[index1] > 0)
          this.RemoveSubPart(this.PercentX[index1]);
        this.totpercent += this.game.Data.LocObj[this.LocNr].ProdPercent[index1];
        index1 += 1;
      }
      while (index1 <= 3);
      this.totpercent = 100 - this.totpercent;
      if (0 > this.totpercent)
        this.totpercent = 0;
      this.NewBackGroundAndClearAll(1024, 200, -1);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      let mut regime: i32 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[this.LocNr].X, this.game.Data.LocObj[this.LocNr].Y].Regime;
      let mut prodslot: i32 = 0;
      SubPartClass tsubpart;
      do
      {
        if (this.slotty == prodslot)
          DrawMod.DrawRectangle( Expression, 0, 3 + prodslot * 44, 351, 44,  this.game.viccolor7.R,  this.game.viccolor7.G,  this.game.viccolor7.B,  this.game.viccolor7.A, 2);
        if (this.game.Data.LocObj[this.LocNr].Production[prodslot] == -1)
        {
          int[] slotButId = this.SlotButId;
          let mut index3: i32 = prodslot;
          tsubpart =  ButtonPartClass::new(this.game.EMPTYSLOT, tResizeX: 40, tresizeY: 30);
          let mut num: i32 = this.AddSubPart( tsubpart, 8, 8 + prodslot * 44, 40, 30, 1);
          slotButId[index3] = num;
        }
        else if (this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[prodslot]].IsResPt & this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[prodslot]].UseSFType == -1)
        {
          int[] slotButId = this.SlotButId;
          let mut index4: i32 = prodslot;
          tsubpart =  ButtonPartClass::new(this.game.PRODPP, tResizeX: 40, tresizeY: 30);
          let mut num: i32 = this.AddSubPart( tsubpart, 8, 8 + prodslot * 44, 40, 30, 1);
          slotButId[index4] = num;
        }
        else if (this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[prodslot]].IsSupply & this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[prodslot]].UseSFType == -1)
        {
          int[] slotButId = this.SlotButId;
          let mut index5: i32 = prodslot;
          tsubpart =  ButtonPartClass::new(this.game.PRODSUPPLY, tResizeX: 40, tresizeY: 30);
          let mut num: i32 = this.AddSubPart( tsubpart, 8, 8 + prodslot * 44, 40, 30, 1);
          slotButId[index5] = num;
        }
        else if (this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[prodslot]].IsSFType > -1 | this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[prodslot]].UseSFType > -1)
        {
          let mut ttyp: i32 = this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[prodslot]].IsSFType;
          if (this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[prodslot]].UseSFType > -1)
            ttyp = this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[prodslot]].UseSFType;
          if (regime > -1)
          {
            if (this.game.Data.RegimeObj[regime].ExtraGraphicUse > -1 & this.game.Data.SFTypeObj[ttyp].ExtraCounter > -1)
            {
              let mut extraCounter: i32 = this.game.Data.SFTypeObj[ttyp].ExtraCounter;
              for (let mut index6: i32 = 0; index6 <= extraCounter; index6 += 1)
              {
                if (this.game.Data.SFTypeObj[ttyp].ExtraCode[index6] == this.game.Data.RegimeObj[regime].ExtraGraphicUse)
                {
                  int[] slotButId = this.SlotButId;
                  let mut index7: i32 = prodslot;
                  tsubpart =  new SFButtonPartClass(ttyp, this.game.Data.RegimeObj[regime].ExtraGraphicUse, 40, 30);
                  let mut num: i32 = this.AddSubPart( tsubpart, 8, 8 + prodslot * 44, 40, 30, 1);
                  slotButId[index7] = num;
                }
              }
            }
            else if (this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ExtraGraphicUse > -1 & this.game.Data.SFTypeObj[ttyp].ExtraCounter > -1)
            {
              let mut extraCounter: i32 = this.game.Data.SFTypeObj[ttyp].ExtraCounter;
              for (let mut index8: i32 = 0; index8 <= extraCounter; index8 += 1)
              {
                if (this.game.Data.SFTypeObj[ttyp].ExtraCode[index8] == this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ExtraGraphicUse)
                {
                  int[] slotButId = this.SlotButId;
                  let mut index9: i32 = prodslot;
                  tsubpart =  new SFButtonPartClass(ttyp, this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ExtraGraphicUse, 40, 30);
                  let mut num: i32 = this.AddSubPart( tsubpart, 8, 8 + prodslot * 44, 40, 30, 1);
                  slotButId[index9] = num;
                }
              }
            }
            else
            {
              int[] slotButId = this.SlotButId;
              let mut index10: i32 = prodslot;
              tsubpart =  new SFButtonPartClass(ttyp, -1, 40, 30);
              let mut num: i32 = this.AddSubPart( tsubpart, 8, 8 + prodslot * 44, 40, 30, 1);
              slotButId[index10] = num;
            }
          }
          else
          {
            int[] slotButId = this.SlotButId;
            let mut index11: i32 = prodslot;
            tsubpart =  new SFButtonPartClass(ttyp, -1, 40, 30);
            let mut num: i32 = this.AddSubPart( tsubpart, 8, 8 + prodslot * 44, 40, 30, 1);
            slotButId[index11] = num;
          }
          DrawMod.DrawRectangle( Expression, 6, 6 + prodslot * 44, 40, 31,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
        }
        txt: String = this.game.Data.LocObj[this.LocNr].Production[prodslot] <= -1 ? "Empty Prod Slot" : Conversion.Str( Conversions.ToInteger(Strings.Trim(Conversion.Str( this.game.Data.LocObj[this.LocNr].TempProdPredict[prodslot])))) + "x " + this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[prodslot]].Name;
        if (this.slotty == prodslot)
        {
          int[] slotText1 = this.SlotText1;
          let mut index12: i32 = prodslot;
          tsubpart =  new ATTextPartClass(txt, this.game.VicFont3, 100, 20, false);
          let mut num: i32 = this.AddSubPart( tsubpart, 50, 5 + prodslot * 44, 97, 20, 0);
          slotText1[index12] = num;
        }
        else
        {
          int[] slotText1 = this.SlotText1;
          let mut index13: i32 = prodslot;
          tsubpart =  new ATTextPartClass(txt, this.game.VicFont3, 100, 20, false);
          let mut num: i32 = this.AddSubPart( tsubpart, 50, 5 + prodslot * 44, 97, 20, 0);
          slotText1[index13] = num;
        }
        if (this.game.Data.LocObj[this.LocNr].Production[prodslot] > -1)
        {
          float estimatedProduction1 =  this.game.HandyFunctionsObj.GetEstimatedProduction(prodslot, this.LocNr, false, false, true);
          int[] slotText2 = this.SlotText2;
          let mut index14: i32 = prodslot;
          tsubpart =  new ATTextPartClass("prd: " + Strings.Trim(Conversion.Str( estimatedProduction1)), this.game.VicFont4, 50, 20, false);
          let mut num1: i32 = this.AddSubPart( tsubpart, 53, 0 + prodslot * 44 + 20, 45, 20, 0);
          slotText2[index14] = num1;
          float single = Conversions.ToSingle(Strings.Trim(Conversion.Str( this.game.Data.LocObj[this.LocNr].TempProdPredict[prodslot])));
          int[] slotText4 = this.SlotText4;
          let mut index15: i32 = prodslot;
          tsubpart =  new ATTextPartClass("real: " + Strings.Trim(Conversion.Str( single)), this.game.VicFont4, 50, 20, false);
          let mut num2: i32 = this.AddSubPart( tsubpart, 53, 12 + prodslot * 44 + 20, 45, 20, 0);
          slotText4[index15] = num2;
          float estimatedProduction2 =  this.game.HandyFunctionsObj.GetEstimatedProduction(prodslot, this.LocNr, false, true, true);
          int[] slotText3 = this.SlotText3;
          let mut index16: i32 = prodslot;
          tsubpart =  new ATTextPartClass("lft: " + Strings.Trim(Conversion.Str( estimatedProduction2)), this.game.VicFont4, 50, 20, false);
          let mut num3: i32 = this.AddSubPart( tsubpart, 100, 0 + prodslot * 44 + 20, 50, 20, 0);
          slotText3[index16] = num3;
        }
        if (this.game.Data.LocObj[this.LocNr].Production[prodslot] > -1)
        {
          let mut index17: i32 = 0;
          do
          {
            if (this.totpercent + this.game.Data.LocObj[this.LocNr].ProdPercent[prodslot] >= index17 * 10)
            {
              if (index17 * 10 != this.game.Data.LocObj[this.LocNr].ProdPercent[prodslot])
              {
                percentBut: Vec<i32> = this.PercentBut;
                let mut index18: i32 = prodslot;
                let mut index19: i32 = index17;
                tsubpart =  ButtonPartClass::new(this.game.PERCENT[index17], 3);
                let mut num: i32 = this.AddSubPart( tsubpart, 155 + index17 * 16, 5 + prodslot * 44 + 4, 16, 32, 1);
                percentBut[index18, index19] = num;
              }
              else
              {
                percentBut: Vec<i32> = this.PercentBut;
                let mut index20: i32 = prodslot;
                let mut index21: i32 = index17;
                tsubpart =  ButtonPartClass::new(this.game.PERCENT[index17]);
                let mut num: i32 = this.AddSubPart( tsubpart, 155 + index17 * 16, 5 + prodslot * 44 + 4, 16, 32, 1);
                percentBut[index20, index21] = num;
              }
            }
            else
            {
              percentBut: Vec<i32> = this.PercentBut;
              let mut index22: i32 = prodslot;
              let mut index23: i32 = index17;
              tsubpart =  ButtonPartClass::new(this.game.PERCENT[index17], 4);
              let mut num: i32 = this.AddSubPart( tsubpart, 155 + index17 * 16, 5 + prodslot * 44 + 4, 16, 32, 0);
              percentBut[index22, index23] = num;
            }
            index17 += 1;
          }
          while (index17 <= 10);
          int[] percentX = this.PercentX;
          let mut index24: i32 = prodslot;
          tsubpart =  ButtonPartClass::new(this.game.PERCENTX);
          let mut num4: i32 = this.AddSubPart( tsubpart, 331, 5 + prodslot * 44 + 4, 16, 32, 1);
          percentX[index24] = num4;
        }
        prodslot += 1;
      }
      while (prodslot <= 3);
      int[] numArray1 = new int[100];
      let mut itemTypeCounter1: i32 = this.game.Data.ItemTypeCounter;
      for (let mut itemtypenr: i32 = 0; itemtypenr <= itemTypeCounter1; itemtypenr += 1)
      {
        if (this.game.HandyFunctionsObj.CanProduceItem(this.LocNr, regime, itemtypenr).result & this.game.Data.ItemTypeObj[itemtypenr].ItemGroup > -1)
        {
          int[] numArray2 = numArray1;
          int[] numArray3 = numArray2;
          let mut itemGroup: i32 = this.game.Data.ItemTypeObj[itemtypenr].ItemGroup;
          let mut index25: i32 = itemGroup;
          let mut num: i32 = numArray2[itemGroup] + 1;
          numArray3[index25] = num;
        }
      }
      if (this.slotty > -1)
      {
        let mut tlistselect1: i32 = -1;
        let mut num5: i32 = -1;
        this.OptionsList2Obj = ATListClass::new();
        if (this.detailnr2 > 99)
          this.detailnr2 = 99;
        let mut tdata: i32 = 0;
        do
        {
          if (numArray1[tdata] > 0)
          {
            this.OptionsList2Obj.add(this.game.Data.TempString[tdata + 300], tdata);
            num5 += 1;
            if (this.detailnr2 == tdata)
              tlistselect1 = num5;
          }
          tdata += 1;
        }
        while (tdata <= 99);
        if (tlistselect1 == -1)
          this.detailnr2 = -1;
        if (this.OptionsList2Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList2Id)].Refresh(this.OptionsList2Obj, tlistselect1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList2Id)] = true;
        }
        else
        {
          tsubpart =  new ATListSubPartClass(this.OptionsList2Obj, 8, 140, tlistselect1, this.game, true, "ItemGroups", tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: 355, bby: 5);
          this.OptionsList2Id = this.AddSubPart( tsubpart, 355, 5, 140, 144, 0);
        }
        if (this.detailnr2 > -1)
        {
          this.OptionsListObj = ATListClass::new();
          if (this.detailnr > this.game.Data.ItemTypeCounter)
            this.detailnr = -1;
          let mut num6: i32 = -1;
          let mut num7: i32 = -1;
          if (this.game.Data.ItemTypeCounter > -1)
          {
            let mut itemTypeCounter2: i32 = this.game.Data.ItemTypeCounter;
            for (let mut index26: i32 = 0; index26 <= itemTypeCounter2; index26 += 1)
            {
              if (this.game.HandyFunctionsObj.CanProduceItem(this.LocNr, regime, index26).result & this.game.Data.ItemTypeObj[index26].ItemGroup == this.detailnr2)
              {
                num7 += 1;
                if (this.detailnr == index26)
                  num6 = num7;
                str1: String = "";
                if (this.game.Data.ItemTypeObj[index26].IsSupply &  this.game.Data.SupplyMultiplier != 1.0)
                  str1 = Strings.Trim(Conversion.Str( this.game.Data.SupplyMultiplier)) + "x ";
                if (this.game.Data.ItemTypeObj[index26].IsResPt &  this.game.Data.PPMultiplier != 1.0)
                  str1 = Strings.Trim(Conversion.Str( this.game.Data.PPMultiplier)) + "x ";
                str2: String = str1 + this.game.Data.ItemTypeObj[index26].Name;
                if (Strings.Len(str2) > 25)
                  str2 = Strings.Left(str2, 25);
                tname: String = str2;
                let mut Number: i32 = this.game.Data.ItemTypeObj[index26].ProdWeight;
                if (this.game.Data.ItemTypeObj[index26].UseProdMod <= 1 &&  this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ProdMod[this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.Data.LocObj[this.LocNr].Map].HexObj[this.game.Data.LocObj[this.LocNr].X, this.game.Data.LocObj[this.LocNr].Y].Regime].People].PeopleGroup] != 0.0)
                  Number =  Math.Round( Conversion.Int( Number / this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ProdMod[this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.Data.LocObj[this.LocNr].Map].HexObj[this.game.Data.LocObj[this.LocNr].X, this.game.Data.LocObj[this.LocNr].Y].Regime].People].PeopleGroup]));
                if (this.game.Data.ItemTypeObj[index26].UseProdMod == 2 &&  this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ProdMod2[this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.Data.LocObj[this.LocNr].Map].HexObj[this.game.Data.LocObj[this.LocNr].X, this.game.Data.LocObj[this.LocNr].Y].Regime].People].PeopleGroup] != 0.0)
                  Number =  Math.Round( Conversion.Int( Number / this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ProdMod2[this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.Data.LocObj[this.LocNr].Map].HexObj[this.game.Data.LocObj[this.LocNr].X, this.game.Data.LocObj[this.LocNr].Y].Regime].People].PeopleGroup]));
                if (this.game.Data.ItemTypeObj[index26].UseProdMod == 3 &&  this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ProdMod3[this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.Data.LocObj[this.LocNr].Map].HexObj[this.game.Data.LocObj[this.LocNr].X, this.game.Data.LocObj[this.LocNr].Y].Regime].People].PeopleGroup] != 0.0)
                  Number =  Math.Round( Conversion.Int( Number / this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ProdMod3[this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.Data.LocObj[this.LocNr].Map].HexObj[this.game.Data.LocObj[this.LocNr].X, this.game.Data.LocObj[this.LocNr].Y].Regime].People].PeopleGroup]));
                if (this.game.Data.ItemTypeObj[index26].UseProdMod == 4 &&  this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ProdMod4[this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.Data.LocObj[this.LocNr].Map].HexObj[this.game.Data.LocObj[this.LocNr].X, this.game.Data.LocObj[this.LocNr].Y].Regime].People].PeopleGroup] != 0.0)
                  Number =  Math.Round( Conversion.Int( Number / this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ProdMod4[this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.Data.LocObj[this.LocNr].Map].HexObj[this.game.Data.LocObj[this.LocNr].X, this.game.Data.LocObj[this.LocNr].Y].Regime].People].PeopleGroup]));
                this.OptionsListObj.add(tname, index26, Strings.Trim(Conversion.Str( Number)));
              }
            }
          }
          this.OptionsListObj.Sort();
          let mut tlistselect2: i32 = -1;
          let mut listCount: i32 = this.OptionsListObj.ListCount;
          for (let mut index27: i32 = 0; index27 <= listCount; index27 += 1)
          {
            if (this.detailnr == this.OptionsListObj.ListData[index27])
              tlistselect2 = index27;
          }
          if (this.OptionsListId > 0)
          {
            this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, tlistselect2);
            this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
          }
          else
          {
            tsubpart =  new ATListSubPartClass(this.OptionsListObj, 8, 300, tlistselect2, this.game, true, "Items", tShowPair: true, tValueWidth: 75, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: 505, bby: 5);
            this.OptionsListId = this.AddSubPart( tsubpart, 505, 5, 300, 144, 0);
          }
          if (this.detailnr > -1 & this.game.Data.LocTypeObj[this.game.Data.LocObj[this.LocNr].Type].AutoProd == -1)
          {
            tsubpart =  new TextButtonPartClass("Select", 96, "Click to assign selected item to production slot",  this.OwnBitmap, 815, 162);
            this.B2Id = this.AddSubPart( tsubpart, 815, 162, 96, 32, 1);
            if (this.game.Data.ItemTypeObj[this.detailnr].IsSFType > -1 & this.game.Data.Round > 0)
            {
              tsubpart =  new TextButtonPartClass("?", 32, "Click to see info on this Subformation Type",  this.OwnBitmap, 915, 162);
              this.Qball = this.AddSubPart( tsubpart, 915, 162, 32, 32, 1);
            }
          }
        }
        if (this.detailnr > -1)
        {
          let mut nr1: i32 = -1;
          let mut nr2: i32 = -1;
          DrawMod.DrawRectangle( Expression, 814, 4, 136, 101,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
          bitmap: Bitmap;
          Rectangle rectangle1;
          Rectangle rectangle2;
          index28: i32;
          num8: i32;
          if (this.game.Data.ItemTypeObj[this.detailnr].IsResPt & this.game.Data.ItemTypeObj[this.detailnr].UseSFType == -1)
          {
             let mut local1: &Graphics = &Expression;
            bitmap = BitmapStore.GetBitmap(this.game.PRODPP);
             let mut local2: &Bitmap = &bitmap;
            rectangle1 = Rectangle::new(0, 0, BitmapStore.GetWidth(this.game.PRODPP), BitmapStore.Getheight(this.game.PRODPP));
            let mut srcrect: &Rectangle = &rectangle1
            rectangle2 = Rectangle::new(815, 5, 135, 100);
            let mut destrect: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
          }
          else if (this.game.Data.ItemTypeObj[this.detailnr].IsSupply & this.game.Data.ItemTypeObj[this.detailnr].UseSFType == -1)
          {
             let mut local3: &Graphics = &Expression;
            bitmap = BitmapStore.GetBitmap(this.game.PRODSUPPLY);
             let mut local4: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(0, 0, BitmapStore.GetWidth(this.game.PRODSUPPLY), BitmapStore.Getheight(this.game.PRODSUPPLY));
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(815, 5, 135, 100);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
          }
          else if (this.game.Data.ItemTypeObj[this.detailnr].IsSFType > -1 | this.game.Data.ItemTypeObj[this.detailnr].UseSFType > -1)
          {
            if (this.game.Data.ItemTypeObj[this.detailnr].UseSFType == -1)
            {
              if (regime > -1)
              {
                if (this.game.Data.RegimeObj[regime].ExtraGraphicUse > -1 & this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].ExtraCounter > -1)
                {
                  let mut extraCounter: i32 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].ExtraCounter;
                  for (let mut index29: i32 = 0; index29 <= extraCounter; index29 += 1)
                  {
                    if (this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].ExtraCode[index29] == this.game.Data.RegimeObj[regime].ExtraGraphicUse)
                    {
                      nr1 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].ExtraPicSpriteID[index29];
                      nr2 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].ExtraSidewaysSpriteID[index29];
                      index28 = this.game.Data.ItemTypeObj[this.detailnr].IsSFType;
                    }
                  }
                }
                else if (this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ExtraGraphicUse > -1 & this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].ExtraCounter > -1)
                {
                  let mut extraCounter: i32 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].ExtraCounter;
                  for (let mut index30: i32 = 0; index30 <= extraCounter; index30 += 1)
                  {
                    if (this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].ExtraCode[index30] == this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ExtraGraphicUse)
                    {
                      nr1 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].ExtraPicSpriteID[index30];
                      nr2 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].ExtraSidewaysSpriteID[index30];
                      index28 = this.game.Data.ItemTypeObj[this.detailnr].IsSFType;
                    }
                  }
                }
                else
                {
                  nr1 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].PicSpriteID;
                  nr2 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].SidewaysSpriteID;
                  index28 = this.game.Data.ItemTypeObj[this.detailnr].IsSFType;
                }
              }
              else
              {
                nr1 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].PicSpriteID;
                nr2 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].IsSFType].SidewaysSpriteID;
                index28 = this.game.Data.ItemTypeObj[this.detailnr].IsSFType;
              }
            }
            else if (regime > -1)
            {
              if (this.game.Data.ItemTypeObj[this.detailnr].UseSFType > -1)
              {
                if (this.game.Data.RegimeObj[regime].ExtraGraphicUse > -1 & this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].ExtraCounter > -1)
                {
                  let mut extraCounter: i32 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].ExtraCounter;
                  for (let mut index31: i32 = 0; index31 <= extraCounter; index31 += 1)
                  {
                    if (this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].ExtraCode[index31] == this.game.Data.RegimeObj[regime].ExtraGraphicUse)
                    {
                      nr1 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].ExtraPicSpriteID[index31];
                      nr2 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].ExtraSidewaysSpriteID[index31];
                      index28 = this.game.Data.ItemTypeObj[this.detailnr].UseSFType;
                      num8 = 1;
                    }
                  }
                }
                else if (this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ExtraGraphicUse > -1 & this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].ExtraCounter > -1)
                {
                  let mut extraCounter: i32 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].ExtraCounter;
                  for (let mut index32: i32 = 0; index32 <= extraCounter; index32 += 1)
                  {
                    if (this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].ExtraCode[index32] == this.game.Data.PeopleObj[this.game.Data.LocObj[this.LocNr].People].ExtraGraphicUse)
                    {
                      nr1 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].ExtraPicSpriteID[index32];
                      nr2 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].ExtraSidewaysSpriteID[index32];
                      index28 = this.game.Data.ItemTypeObj[this.detailnr].UseSFType;
                      num8 = 1;
                    }
                  }
                }
                else if (this.game.Data.ItemTypeObj[this.detailnr].UseSFType > -1)
                {
                  nr1 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].PicSpriteID;
                  nr2 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].SidewaysSpriteID;
                  index28 = this.game.Data.ItemTypeObj[this.detailnr].UseSFType;
                  num8 = 1;
                }
              }
              else
              {
                nr1 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].PicSpriteID;
                nr2 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].SidewaysSpriteID;
                index28 = this.game.Data.ItemTypeObj[this.detailnr].UseSFType;
                num8 = 1;
              }
            }
            else
            {
              nr1 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].PicSpriteID;
              nr2 = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.detailnr].UseSFType].SidewaysSpriteID;
              index28 = this.game.Data.ItemTypeObj[this.detailnr].UseSFType;
              num8 = 1;
            }
          }
           let mut local5: &Graphics = &Expression;
          rectangle2 = Rectangle::new(815, 110, 136, 14);
          let mut rect1: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(815, 124, 136, 23);
          let mut rect2: &Rectangle = &rectangle1
          name: String = this.game.Data.ItemTypeObj[this.detailnr].Name;
          DrawMod.MakeFullBoxVic2( local5, rect1, "ITEM TYPE", rect2, name);
          x1: i32;
          y1: i32;
          width1: i32;
          height: i32;
          index33: i32;
          index34: i32;
          if (nr1 > -1)
          {
            let mut index35: i32 = this.game.Data.Turn;
            if (index35 == -1)
            {
              index35 = this.game.Data.MapObj[0].HexObj[this.game.Data.LocObj[this.LocNr].X, this.game.Data.LocObj[this.LocNr].Y].Regime;
              if (this.game.Data.RegimeCounter == -1)
                return;
              if (index35 == -1)
                index35 = 0;
            }
            let mut red: i32 = this.game.Data.RegimeObj[index35].Red;
            let mut green: i32 = this.game.Data.RegimeObj[index35].Green;
            let mut blue: i32 = this.game.Data.RegimeObj[index35].Blue;
            let mut baseColor: i32 = this.game.Data.SFTypeObj[index28].BaseColor;
            x1 = 815;
            y1 = 5;
            width1 = 135;
            height = 100;
            if ( this.game.Data.RuleVar[869] >= 1.0 & num8 == 0)
            {
              index33 =  Math.Round( this.game.Data.RuleVar[873]);
              index34 = 0;
              if ( this.game.Data.RuleVar[848] > 0.0 & this.game.Data.SFTypeObj[index28].Theater == 2)
              {
                index33 =  Math.Round( this.game.Data.RuleVar[848]);
                index34 = 0;
              }
              if ( this.game.Data.RuleVar[872] > 0.0 & this.game.Data.SFTypeObj[index28].Theater == 1)
              {
                index33 =  Math.Round( this.game.Data.RuleVar[872]);
                index34 = 0;
              }
              if ( this.game.Data.RuleVar[869] == 3.0)
              {
                let mut nr3: i32 = this.game.Data.LandscapeTypeObj[index33].BasicPicID[index34];
                 let mut local6: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(nr3);
                 let mut local7: &Bitmap = &bitmap;
                rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr3));
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(x1, y1, width1, height);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2( local6,  local7, srcrect, destrect);
              }
              else
              {
                if ( this.game.Data.RuleVar[869] == 1.0)
                {
                  let mut nr4: i32 = this.game.Data.LandscapeTypeObj[index33].SidewaysSPriteID1[index34];
                   let mut local8: &Graphics = &Expression;
                  bitmap = BitmapStore.GetBitmap(nr4);
                   let mut local9: &Bitmap = &bitmap;
                  rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr4));
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(x1, y1, width1, height);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2( local8,  local9, srcrect, destrect);
                }
                let mut nr5: i32 = this.game.Data.LandscapeTypeObj[index33].SidewaysSPriteID2[index34];
                 let mut local10: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(nr5);
                 let mut local11: &Bitmap = &bitmap;
                rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr5));
                let mut srcrect1: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(x1, y1, width1, height);
                let mut destrect1: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2( local10,  local11, srcrect1, destrect1);
              }
            }
            switch (baseColor)
            {
              case 0:
                 let mut local12: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(nr1);
                 let mut local13: &Bitmap = &bitmap;
                let mut x2: i32 = x1;
                let mut y2: i32 = y1;
                let mut w1: i32 = width1;
                let mut h1: i32 = height;
                DrawMod.DrawScaled( local12,  local13, x2, y2, w1, h1);
                break;
              case 1:
                 let mut local14: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(nr1);
                 let mut local15: &Bitmap = &bitmap;
                let mut x3: i32 = x1;
                let mut y3: i32 = y1;
                let mut w2: i32 = width1;
                let mut h2: i32 = height;
                let mut width2: i32 = BitmapStore.GetWidth(nr1);
                let mut origh1: i32 = BitmapStore.Getheight(nr1);
                double r1 =  ( red / 256f);
                double g1 =  ( green / 256f);
                double b1 =  ( blue / 256f);
                DrawMod.DrawScaledColorized2( local14,  local15, x3, y3, w2, h2, width2, origh1,  r1,  g1,  b1, 1f);
                break;
              case 2:
                let mut red2: i32 = this.game.Data.RegimeObj[index35].Red2;
                let mut green2: i32 = this.game.Data.RegimeObj[index35].Green2;
                let mut blue2: i32 = this.game.Data.RegimeObj[index35].Blue2;
                 let mut local16: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(nr1);
                 let mut local17: &Bitmap = &bitmap;
                let mut x4: i32 = x1;
                let mut y4: i32 = y1;
                let mut w3: i32 = width1;
                let mut h3: i32 = height;
                let mut width3: i32 = BitmapStore.GetWidth(nr1);
                let mut origh2: i32 = BitmapStore.Getheight(nr1);
                double r2 =  ( red2 / 256f);
                double g2 =  ( green2 / 256f);
                double b2 =  ( blue2 / 256f);
                DrawMod.DrawScaledColorized2( local16,  local17, x4, y4, w3, h3, width3, origh2,  r2,  g2,  b2, 1f);
                break;
              case 3:
                let mut red3: i32 = this.game.Data.RegimeObj[index35].Red3;
                let mut green3: i32 = this.game.Data.RegimeObj[index35].Green3;
                let mut blue3: i32 = this.game.Data.RegimeObj[index35].Blue3;
                 let mut local18: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(nr1);
                 let mut local19: &Bitmap = &bitmap;
                let mut x5: i32 = x1;
                let mut y5: i32 = y1;
                let mut w4: i32 = width1;
                let mut h4: i32 = height;
                let mut width4: i32 = BitmapStore.GetWidth(nr1);
                let mut origh3: i32 = BitmapStore.Getheight(nr1);
                double r3 =  ( red3 / 256f);
                double g3 =  ( green3 / 256f);
                double b3 =  ( blue3 / 256f);
                DrawMod.DrawScaledColorized2( local18,  local19, x5, y5, w4, h4, width4, origh3,  r3,  g3,  b3, 1f);
                break;
              case 4:
                let mut red4: i32 = this.game.Data.RegimeObj[index35].Red4;
                let mut green4: i32 = this.game.Data.RegimeObj[index35].Green4;
                let mut blue4: i32 = this.game.Data.RegimeObj[index35].Blue4;
                 let mut local20: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(nr1);
                 let mut local21: &Bitmap = &bitmap;
                let mut x6: i32 = x1;
                let mut y6: i32 = y1;
                let mut w5: i32 = width1;
                let mut h5: i32 = height;
                let mut width5: i32 = BitmapStore.GetWidth(nr1);
                let mut origh4: i32 = BitmapStore.Getheight(nr1);
                double r4 =  ( red4 / 256f);
                double g4 =  ( green4 / 256f);
                double b4 =  ( blue4 / 256f);
                DrawMod.DrawScaledColorized2( local20,  local21, x6, y6, w5, h5, width5, origh4,  r4,  g4,  b4, 1f);
                break;
              case 5:
                 let mut local22: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(nr1);
                 let mut local23: &Bitmap = &bitmap;
                let mut x7: i32 = x1;
                let mut y7: i32 = y1;
                let mut w6: i32 = width1;
                let mut h6: i32 = height;
                let mut width6: i32 = BitmapStore.GetWidth(nr1);
                let mut origh5: i32 = BitmapStore.Getheight(nr1);
                double r5 =  ( (red + 392) / 1024f);
                double g5 =  ( (green + 392) / 1024f);
                double b5 =  ( (blue + 392) / 1024f);
                DrawMod.DrawScaledColorized2( local22,  local23, x7, y7, w6, h6, width6, origh5,  r5,  g5,  b5, 1f);
                break;
              case 6:
                 let mut local24: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(nr1);
                 let mut local25: &Bitmap = &bitmap;
                let mut x8: i32 = x1;
                let mut y8: i32 = y1;
                let mut w7: i32 = width1;
                let mut h7: i32 = height;
                let mut width7: i32 = BitmapStore.GetWidth(nr1);
                let mut origh6: i32 = BitmapStore.Getheight(nr1);
                double r6 =  ( (red + 80) / 512f);
                double g6 =  ( (green + 200) / 512f);
                double b6 =  ( (blue + 80) / 512f);
                DrawMod.DrawScaledColorized2( local24,  local25, x8, y8, w7, h7, width7, origh6,  r6,  g6,  b6, 1f);
                break;
            }
          }
          if (nr2 > -1 &&  this.game.Data.RuleVar[870] > 0.0 & !Information.IsNothing( BitmapStore.GetBitmap(nr2)))
          {
             let mut local26: &Graphics = &Expression;
            bitmap = BitmapStore.GetBitmap(nr2);
             let mut local27: &Bitmap = &bitmap;
            let mut x9: i32 = x1;
            let mut y9: i32 = y1;
            let mut w: i32 = width1;
            let mut h: i32 = height;
            DrawMod.DrawScaled( local26,  local27, x9, y9, w, h);
          }
          if ( this.game.Data.RuleVar[869] >= 1.0 &  this.game.Data.RuleVar[869] < 3.0)
          {
            let mut nr6: i32 = this.game.Data.LandscapeTypeObj[index33].SidewaysSPriteID3[index34];
             let mut local28: &Graphics = &Expression;
            bitmap = BitmapStore.GetBitmap(nr6);
             let mut local29: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr6));
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(x1, y1, 192, 144);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local28,  local29, srcrect, destrect);
          }
          let mut num9: i32 = -1;
          let mut index36: i32 = 0;
          do
          {
            if (this.game.Data.ItemTypeObj[this.detailnr].RegimeSlotsCost[index36] > -1)
            {
              num9 += 1;
              tstring: String = Strings.Trim(Conversion.Str( this.game.Data.ItemTypeObj[this.detailnr].RegimeSlotsCostQty[index36])) + "x " + this.game.Data.RegimeSlotName[this.game.Data.ItemTypeObj[this.detailnr].RegimeSlotsCost[index36]];
              let mut num10: i32 = 90 - num9 * 15;
              DrawMod.DrawBlock( Expression, 815, num10, 135, 15, 0, 0, 0, 125);
              DrawMod.DrawTextVic2( Expression, tstring, this.game.VicFont2, 820, num10, this.game.VicColor2, this.game.VicColor2Shade);
            }
            index36 += 1;
          }
          while (index36 <= 4);
        }
      }
      if (regime > -1)
      {
        Color.FromArgb( byte.MaxValue, 180, 200, 240);
        Color.FromArgb( byte.MaxValue, 120, 120, 160);
        Color.FromArgb( byte.MaxValue, 200, 200, 200);
        Color.FromArgb( byte.MaxValue, 130, 130, 130);
        white: Color = Color.White;
        black: Color = Color.Black;
        font: Font = Font::new("Arial", 13f, FontStyle.Bold, GraphicsUnit.Pixel);
        if (this.game.Data.Round > 0)
        {
          tsubpart =  new TextButtonPartClass(!this.game.EditObj.ProdFlap ? "See Production Overview" : "Close Production Overview", 200, tBackbitmap: ( this.OwnBitmap), bbx: 355, bby: 162);
          this.FlapBall = this.AddSubPart( tsubpart, 355, 162, 200, 35, 1);
          if (!this.game.Data.LocTypeObj[this.game.Data.LocObj[this.LocNr].Type].NoHQ)
          {
            tsubpart =  new TextButtonPartClass("Select HQ", 100, tBackbitmap: ( this.OwnBitmap), bbx: 585, bby: 162);
            this.HqBall = this.AddSubPart( tsubpart, 585, 162, 100, 35, 1);
          }
        }
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = this.SubPartCounter;
label_60:
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 = this.SubPartID[index1];
            if (num1 == this.B2Id)
            {
              if (this.detailnr > -1)
              {
                if (this.game.Data.LocObj[this.LocNr].ProdPointRemainder[this.slotty] > 0 && Interaction.MsgBox( "Are you sure you want to change production? You will lose left over production currently stored.", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.No)
                  return windowReturnClass;
                this.game.Data.LocObj[this.LocNr].Production[this.slotty] = this.detailnr;
                this.game.Data.LocObj[this.LocNr].ProdPointRemainder[this.slotty] = 0;
                this.detailnr = this.game.Data.LocObj[this.LocNr].Production[this.slotty];
                this.detailnr2 = this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[this.slotty]].ItemGroup;
                windowReturnClass.AddCommand(4, 44);
                if (this.game.EditObj.ProdFlap)
                  windowReturnClass.AddCommand(4, 52);
                this.FixProdPercent2(this.slotty);
                this.game.ProcessingObj.LocationProductionPrognosis();
                if (!this.game.EditObj.ProdFlap)
                  windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(4, 66);
                windowReturnClass.AddCommand(4, 29);
              }
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.FlapBall)
            {
              if (!this.game.EditObj.ProdFlap)
              {
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 3);
                windowReturnClass.AddCommand(2, 52);
                windowReturnClass.AddCommand(1, 66);
                windowReturnClass.AddCommand(2, 66);
                windowReturnClass.AddCommand(4, 29);
                this.game.EditObj.ProdFlap = true;
              }
              else
              {
                windowReturnClass.AddCommand(1, 3);
                windowReturnClass.AddCommand(2, 18);
                windowReturnClass.AddCommand(2, 12);
                windowReturnClass.AddCommand(1, 66);
                windowReturnClass.AddCommand(2, 66);
                windowReturnClass.AddCommand(4, 29);
                this.game.EditObj.ProdFlap = false;
              }
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.HqBall)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 33, this.LocNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.Qball)
            {
              this.game.EditObj.TempProdList1 = this.detailnr;
              this.game.EditObj.TempProdList2 = this.detailnr2;
              this.game.EditObj.TempProdList3 = this.slotty;
              this.game.EditObj.SFTypeSelected = this.game.Data.ItemTypeObj[this.detailnr].IsSFType;
              this.game.EditObj.SFSelected = -1;
              this.game.EditObj.PopupValue = 6;
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.AddCommand(5, 10);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsListId)
            {
              let mut num2: i32 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
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
            if (num1 == this.OptionsList2Id)
            {
              let mut num3: i32 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num3 > -1)
              {
                this.detailnr2 = num3;
                this.dostuff();
              }
              if (num3 == -2)
              {
                this.detailnr2 = -1;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.LocNr].Type].AutoProd == -1)
            {
              let mut index2: i32 = 0;
label_36:
              let mut num4: i32 = this.SubPartID[index1];
              if (num4 == this.SlotButId[index2])
              {
                this.slotty = index2;
                if (this.game.Data.LocObj[this.LocNr].Production[this.slotty] > -1)
                {
                  this.detailnr = this.game.Data.LocObj[this.LocNr].Production[this.slotty];
                  this.detailnr2 = this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[this.slotty]].ItemGroup;
                }
                this.dostuff();
                if (this.game.EditObj.ProdFlap)
                  windowReturnClass.AddCommand(4, 52);
                windowReturnClass.AddCommand(4, 66);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num4 == this.PercentX[index2] && Interaction.MsgBox( "Are you sure to delete this slot? You'll lose all remaining production stored in this slot.", MsgBoxStyle.YesNo,  "Check") == MsgBoxResult.Yes)
              {
                this.game.Data.LocObj[this.LocNr].ProdPercent[index2] = 0;
                this.game.Data.LocObj[this.LocNr].Production[index2] = -1;
                this.game.Data.LocObj[this.LocNr].ProdPointRemainder[index2] = 0;
                this.slotty = index2;
                this.detailnr = -1;
                this.detailnr2 = -1;
                this.game.ProcessingObj.LocationProductionPrognosis();
                this.dostuff();
                if (!this.game.EditObj.ProdFlap)
                  windowReturnClass.AddCommand(4, 18);
                windowReturnClass.SetFlag(true);
                if (this.game.EditObj.ProdFlap)
                  windowReturnClass.AddCommand(4, 52);
                windowReturnClass.AddCommand(4, 66);
                return windowReturnClass;
              }
              let mut index3: i32 = 0;
              while (this.PercentBut[index2, index3] != this.SubPartID[index1])
              {
                index3 += 1;
                if (index3 > 10)
                {
                  index2 += 1;
                  if (index2 <= 3)
                    goto label_36;
                  else
                    goto label_60;
                }
              }
              if (this.totpercent + this.game.Data.LocObj[this.LocNr].ProdPercent[index2] >= index3 * 10)
              {
                this.game.Data.LocObj[this.LocNr].ProdPercent[index2] = index3 * 10;
                this.slotty = index2;
                this.detailnr = this.game.Data.LocObj[this.LocNr].Production[index2];
                this.detailnr2 = this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[index2]].ItemGroup;
              }
              this.game.ProcessingObj.LocationProductionPrognosis();
              this.dostuff();
              if (!this.game.EditObj.ProdFlap)
                windowReturnClass.AddCommand(4, 18);
              if (this.game.EditObj.ProdFlap)
                windowReturnClass.AddCommand(4, 52);
              windowReturnClass.AddCommand(4, 66);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
        }
        if (x < 330 && y < 176)
        {
          this.slotty =  Math.Round(Conversion.Int( y / 44.0));
          if (this.slotty > 3)
            this.slotty = 3;
          if (this.game.Data.LocObj[this.LocNr].Production[this.slotty] > -1)
          {
            this.detailnr = this.game.Data.LocObj[this.LocNr].Production[this.slotty];
            this.detailnr2 = this.game.Data.ItemTypeObj[this.game.Data.LocObj[this.LocNr].Production[this.slotty]].ItemGroup;
          }
          this.dostuff();
          if (this.game.EditObj.ProdFlap)
            windowReturnClass.AddCommand(4, 52);
          windowReturnClass.AddCommand(4, 66);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn FixProdPercent(tempslotty: i32)
    {
      int[] numArray1 = new int[4];
      int[] numArray2 = new int[4];
      let mut num1: i32 = 0;
      let mut index1: i32 = 0;
      do
      {
        numArray1[index1] = this.game.Data.LocObj[this.LocNr].ProdPercent[index1];
        num1 += numArray1[index1];
        index1 += 1;
      }
      while (index1 <= 3);
      if (num1 <= 100)
        return;
      let mut index2: i32 = -1;
      while (num1 > 100)
      {
        index2 += 1;
        if (index2 > 3)
          index2 = 0;
        if (index2 != tempslotty && this.game.Data.LocObj[this.LocNr].Production[index2] > -1 && this.game.Data.LocObj[this.LocNr].ProdPercent[index2] > 0)
        {
          int[] prodPercent = this.game.Data.LocObj[this.LocNr].ProdPercent;
          int[] numArray3 = prodPercent;
          let mut index3: i32 = index2;
          let mut index4: i32 = index3;
          let mut num2: i32 = prodPercent[index3] - 1;
          numArray3[index4] = num2;
          --num1;
        }
      }
    }

    pub fn FixProdPercent2(tempslotty: i32)
    {
      int[] numArray1 = new int[4];
      int[] numArray2 = new int[4];
      let mut num: i32 = 0;
      let mut index: i32 = 0;
      do
      {
        numArray1[index] = this.game.Data.LocObj[this.LocNr].ProdPercent[index];
        num += numArray1[index];
        index += 1;
      }
      while (index <= 3);
      if (num >= 100)
        return;
      this.game.Data.LocObj[this.LocNr].ProdPercent[tempslotty] = 100 - num;
    }
  }
}
