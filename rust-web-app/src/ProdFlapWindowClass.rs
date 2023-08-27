// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ProdFlapWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class ProdFlapWindowClass : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     b1textid: i32;
     b2textid: i32;
     curheight: i32;
     detailnr: i32;
     butup: i32;
     OptionsListId: i32;
     ATListClass OptionsListObj;
     OptionsList2Id: i32;
     ATListClass OptionsList2Obj;
     detailnr2: i32;
     butdown: i32;
     int[] butpage;
     int[] butdata;
     butcount: i32;
     curbut: i32;
     butres: i32;
     resmode: i32;
     txt1id: i32;
     txt2id: i32;
     txt3id: i32;
     txt4id: i32;
     txt5id: i32;
     txt6id: i32;
     txt7id: i32;
     txt8id: i32;
     int[] mzx;
     int[] mzy;
     int[] mzx2;
     int[] mzy2;
     int[] mznr;
     mzcount: i32;

    pub ProdFlapWindowClass(
       tGame: GameClass,
      theight: i32,
      screenbitmap: Bitmap = null,
      let mut sx: i32 = -1,
      let mut sy: i32 = -1,
      let mut tl1: i32 = -1,
      let mut tl2: i32 = -1,
      let mut tl3: i32 = -1)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - theight, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.butpage = new int[1000];
      this.butdata = new int[1000];
      this.mzx = new int[100];
      this.mzy = new int[100];
      this.mzx2 = new int[100];
      this.mzy2 = new int[100];
      this.mznr = new int[100];
      this.curheight = tGame.ScreenHeight - theight;
      this.mapframe = true;
      this.detailnr = 0;
      this.detailnr2 = -1;
      this.dostuff();
    }

    pub fn DoRefresh() => this.dostuff();

    pub fn dostuff()
    {
      float[] numArray1 = new float[this.game.Data.ItemTypeCounter + 1];
      let mut num1: i32 = 0;
      let mut num2: i32 = 0;
      if (this.butup > 0)
        this.RemoveSubPart(this.butup);
      if (this.butdown > 0)
        this.RemoveSubPart(this.butdown);
      if (this.butres > 0)
        this.RemoveSubPart(this.butres);
      let mut index1: i32 = 0;
      do
      {
        if (this.butpage[index1] > 0)
        {
          this.RemoveSubPart(this.butpage[index1]);
          this.butpage[index1] = 0;
        }
        index1 += 1;
      }
      while (index1 <= 100);
      let mut index2: i32 = 0;
      do
      {
        this.mzx[index2] = 0;
        this.mzx2[index2] = 0;
        this.mzy[index2] = 0;
        this.mzy2[index2] = 0;
        this.mznr[index2] = 0;
        index2 += 1;
      }
      while (index2 <= 99);
      this.mzcount = -1;
      this.NewBackGroundAndClearAll(this.game.ScreenWidth, this.curheight, -1);
      SimpleList simpleList1 = SimpleList::new();
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      let mut num3: i32 = 20;
      let mut num4: i32 =  Math.Round( (this.game.ScreenWidth - 1080) / 2.0);
      if (num4 < 0)
        num4 = 0;
      let mut num5: i32 = num3 + num4;
      DrawMod.DrawTextVic2( graphics, "HEADQUARTER", this.game.VicFont1, num5 + 10, 25, this.game.VicColor1, this.game.VicColor1Shade);
      DrawMod.DrawTextVic2( graphics, "LOCATION", this.game.VicFont1, num5 + 200, 25, this.game.VicColor1, this.game.VicColor1Shade);
      if (this.resmode == 0)
        DrawMod.DrawTextVic2( graphics, "PRODUCTION", this.game.VicFont1, num5 + 450, 25, this.game.VicColor1, this.game.VicColor1Shade);
      else
        DrawMod.DrawTextVic2( graphics, "RESOURCE USE", this.game.VicFont1, num5 + 450, 25, this.game.VicColor1, this.game.VicColor1Shade);
      SimpleList simpleList2 = SimpleList::new();
      let mut mapCounter: i32 = this.game.Data.MapCounter;
      for (let mut tdata4: i32 = 0; tdata4 <= mapCounter; tdata4 += 1)
      {
        let mut mapWidth: i32 = this.game.Data.MapObj[tdata4].MapWidth;
        for (let mut tdata1: i32 = 0; tdata1 <= mapWidth; tdata1 += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[tdata4].MapHeight;
          for (let mut tdata2: i32 = 0; tdata2 <= mapHeight; tdata2 += 1)
          {
            if (this.game.Data.MapObj[tdata4].HexObj[tdata1, tdata2].Regime == this.game.Data.Turn && this.game.Data.MapObj[tdata4].HexObj[tdata1, tdata2].Location > -1)
            {
              let mut location: i32 = this.game.Data.MapObj[tdata4].HexObj[tdata1, tdata2].Location;
              let mut maxProd: i32 = this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].MaxProd;
              if (this.game.HandyFunctionsObj.CanLocProduce(location, this.game.Data.Turn) && maxProd > 0 && this.detailnr2 == -1 & this.game.Data.LocObj[location].HQ > -1 | this.detailnr2 == -2 & this.game.Data.LocObj[location].HQ == -1 | this.detailnr2 == this.game.Data.LocObj[location].HQ & this.game.Data.LocObj[location].HQ > -1)
                simpleList2.Add(location, this.game.Data.LocObj[location].HQ, tdata1, tdata2, this.game.Data.LocObj[location].Type, tdata4);
            }
          }
        }
      }
      this.butcount =  Math.Round( (this.curheight - 110) / 50.0);
      num6: i32;
      if (simpleList2.Counter > -1)
      {
        simpleList2.Sort();
        let mut num7: i32 = -1;
        let mut num8: i32 = -1;
        let mut num9: i32 = 0;
        let mut index3: i32 = 1;
        for (let mut counter: i32 = simpleList2.Counter; counter >= 0; counter += -1)
        {
          num8 += 1;
          num9 += 1;
          if (num9 > this.butcount)
          {
            num9 = 1;
            index3 += 1;
          }
          if (num9 == 1)
            this.butdata[index3] = num8;
          if (num8 == this.detailnr)
            this.curbut = index3;
          let mut num10: i32 = 0;
          if (num8 >= this.detailnr)
          {
            num7 += 1;
            if (num7 * 50 + 30 + 50 + 30 + 0 <= this.curheight)
            {
              num10 = 1;
              let mut index4: i32 = simpleList2.Id[counter];
              let mut nr1: i32 = simpleList2.Weight[counter];
              let mut num11: i32 = simpleList2.Data1[counter];
              let mut num12: i32 = simpleList2.Data2[counter];
              num6 = simpleList2.Data3[counter];
              let mut num13: i32 = simpleList2.Data4[counter];
              let mut num14: i32 = 50 + 50 * num7;
              let mut regime: i32 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[index4].X, this.game.Data.LocObj[index4].Y].Regime;
              DrawMod.DrawBlock( graphics, num5 - 5, num14 - 1, 685 + num4 - num5, 49,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
              if (index4 == this.game.EditObj.OrderLoc)
                DrawMod.DrawRectangle( graphics, num5 - 5, num14 - 1, 685 + num4 - num5, 49,  this.game.viccolor7.R,  this.game.viccolor7.G,  this.game.viccolor7.B,  this.game.viccolor7.A, 2);
              else
                DrawMod.DrawRectangle( graphics, num5 - 5, num14 - 1, 685 + num4 - num5, 49,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
              if (nr1 != -1)
              {
                this.game.CustomBitmapObj.DrawUnit(nr1, toG: graphics, tx: (num5 + 5), ty: (num14 + 5));
                DrawMod.DrawTextVic2( graphics, this.game.Data.UnitObj[nr1].Name, this.game.VicFont3, num5 + 45, num14 + 15, this.game.VicColor2, this.game.VicColor2Shade);
              }
               let mut local1: &Graphics = &graphics;
              customBitmapObj: CustomBitmapClass = this.game.CustomBitmapObj;
              let mut cx: i32 = num11;
              let mut cy: i32 = num12;
              let mut cmap: i32 = num13;
              bitmap1: Bitmap = (Bitmap) null;
               let mut local2: &Bitmap = &bitmap1;
              bool flag = false;
               bool local3 =  flag;
              bitmap2: Bitmap = customBitmapObj.DrawHex(cx, cy, cmap, true, gBitmap: ( local2), tFromMapPopup: ( local3));
               let mut local4: &Bitmap = &bitmap2;
              let mut x1: i32 = num5 + 200;
              let mut y1: i32 = num14;
              DrawMod.DrawSimple( local1,  local4, x1, y1);
              DrawMod.DrawTextVic2( graphics, this.game.Data.LocObj[index4].Name, this.game.VicFont2, num5 + 268, num14 + 15, this.game.VicColor2, this.game.VicColor2Shade);
              let mut index5: i32 = 0;
              do
              {
                if (this.game.Data.LocObj[index4].Production[index5] > -1)
                {
                  let mut index6: i32 = this.game.Data.LocObj[index4].Production[index5];
                  if (this.resmode == 0)
                  {
                    tstring: String = Strings.Left(this.game.Data.ItemTypeObj[index6].Name, 3);
                    if (this.game.Data.ItemTypeObj[index6].IsSupply)
                      tstring = "SU";
                    if (this.game.Data.ItemTypeObj[index6].IsResPt)
                      tstring = "PP";
                    if (this.game.Data.ItemTypeObj[index6].IsSFType > -1 & regime > -1)
                    {
                      symbolSpriteId: i32;
                      if (this.game.Data.RegimeObj[regime].ExtraGraphicUse > -1 & this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[index6].IsSFType].ExtraCounter > -1 & this.game.Data.Round > 0)
                      {
                        let mut isSfType: i32 = this.game.Data.ItemTypeObj[index6].IsSFType;
                        let mut extraCounter: i32 = this.game.Data.SFTypeObj[isSfType].ExtraCounter;
                        for (let mut index7: i32 = 0; index7 <= extraCounter; index7 += 1)
                        {
                          if (this.game.Data.SFTypeObj[isSfType].ExtraCode[index7] == this.game.Data.RegimeObj[regime].ExtraGraphicUse)
                            symbolSpriteId = this.game.Data.SFTypeObj[isSfType].ExtraSymbolSpriteID[index7];
                        }
                      }
                      else if (this.game.Data.PeopleObj[this.game.Data.LocObj[index4].People].ExtraGraphicUse > -1 & this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[index6].IsSFType].ExtraCounter > -1)
                      {
                        let mut isSfType: i32 = this.game.Data.ItemTypeObj[index6].IsSFType;
                        let mut extraCounter: i32 = this.game.Data.SFTypeObj[isSfType].ExtraCounter;
                        for (let mut index8: i32 = 0; index8 <= extraCounter; index8 += 1)
                        {
                          if (this.game.Data.SFTypeObj[isSfType].ExtraCode[index8] == this.game.Data.PeopleObj[this.game.Data.LocObj[index4].People].ExtraGraphicUse)
                            symbolSpriteId = this.game.Data.SFTypeObj[isSfType].ExtraSymbolSpriteID[index8];
                        }
                      }
                      else
                        symbolSpriteId = this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[index6].IsSFType].SymbolSpriteID;
                       let mut local5: &Graphics = &graphics;
                      bitmap3: Bitmap = BitmapStore.GetBitmap(symbolSpriteId);
                       let mut local6: &Bitmap = &bitmap3;
                      let mut x2: i32 = num5 + 440 + index5 * 50;
                      let mut y2: i32 = num14 + 5;
                      DrawMod.DrawSimple( local5,  local6, x2, y2);
                    }
                    else
                      DrawMod.DrawTextVic2( graphics, tstring, this.game.VicFont1, num5 + 455 + index5 * 50, num14 + 4, this.game.VicColor1, this.game.VicColor1Shade);
                  }
                  float Number = Conversion.Int(Conversions.ToSingle(Strings.Trim(Conversion.Str( this.game.Data.LocObj[index4].TempProdPredict[index5]))));
                  float[] numArray2 = numArray1;
                  float[] numArray3 = numArray2;
                  let mut index9: i32 = index6;
                  let mut index10: i32 = index9;
                  double num15 =  numArray2[index9] +  Number;
                  numArray3[index10] =  num15;
                  if (this.resmode == 0)
                    DrawMod.DrawTextVic2( graphics, Strings.Trim(Conversion.Str( Number)), this.game.VicFont3, num5 + 455 + index5 * 50, num14 + 27, this.game.VicColor2, this.game.VicColor2Shade);
                  let mut index11: i32 = 0;
                  do
                  {
                    let mut index12: i32 = this.game.Data.LocObj[index4].Production[index5];
                    if (index12 > -1 && this.game.Data.ItemTypeObj[index12].RegimeSlotsCost[index11] > -1)
                    {
                      let mut nr2: i32 = simpleList1.FindNr(this.game.Data.ItemTypeObj[index12].RegimeSlotsCost[index11]);
                      if (nr2 == -1)
                      {
                        simpleList1.Add(this.game.Data.ItemTypeObj[index12].RegimeSlotsCost[index11],  Math.Round( ( this.game.Data.ItemTypeObj[index12].RegimeSlotsCostQty[index11] * Number)));
                      }
                      else
                      {
                        int[] weight = simpleList1.Weight;
                        int[] numArray4 = weight;
                        let mut index13: i32 = nr2;
                        let mut index14: i32 = index13;
                        let mut num16: i32 =  Math.Round( ( weight[index13] +  this.game.Data.ItemTypeObj[index12].RegimeSlotsCostQty[index11] * Number));
                        numArray4[index14] = num16;
                      }
                    }
                    index11 += 1;
                  }
                  while (index11 <= 4);
                }
                index5 += 1;
              }
              while (index5 <= 3);
              this += 1.mzcount;
              this.mzx[this.mzcount] = num5;
              this.mzx2[this.mzcount] = num5 + 660;
              this.mzy[this.mzcount] = num14;
              this.mzy2[this.mzcount] = num14 + 49;
              this.mznr[this.mzcount] = index4;
            }
            else
              num2 = 1;
          }
          else
            num1 = 1;
          if (num10 == 0)
          {
            let mut locnr: i32 = simpleList2.Id[counter];
            let mut prodslot: i32 = 0;
            do
            {
              if (this.game.Data.LocObj[locnr].Production[prodslot] > -1)
              {
                let mut num17: i32 = this.game.Data.LocObj[locnr].Production[prodslot];
                float num18 = Conversion.Int( this.game.HandyFunctionsObj.GetEstimatedProduction(prodslot, locnr, true, false));
                float[] numArray5 = numArray1;
                float[] numArray6 = numArray5;
                let mut index15: i32 = num17;
                let mut index16: i32 = index15;
                double num19 =  numArray5[index15] +  num18;
                numArray6[index16] =  num19;
                let mut index17: i32 = 0;
                do
                {
                  let mut index18: i32 = this.game.Data.LocObj[locnr].Production[prodslot];
                  if (index18 > -1 && this.game.Data.ItemTypeObj[index18].RegimeSlotsCost[index17] > -1)
                  {
                    let mut nr: i32 = simpleList1.FindNr(this.game.Data.ItemTypeObj[index18].RegimeSlotsCost[index17]);
                    if (nr == -1)
                    {
                      simpleList1.Add(this.game.Data.ItemTypeObj[index18].RegimeSlotsCost[index17],  Math.Round( ( this.game.Data.ItemTypeObj[index18].RegimeSlotsCostQty[index17] * num18)));
                    }
                    else
                    {
                      int[] weight = simpleList1.Weight;
                      int[] numArray7 = weight;
                      let mut index19: i32 = nr;
                      let mut index20: i32 = index19;
                      let mut num20: i32 =  Math.Round( ( weight[index19] +  this.game.Data.ItemTypeObj[index18].RegimeSlotsCostQty[index17] * num18));
                      numArray7[index20] = num20;
                    }
                  }
                  index17 += 1;
                }
                while (index17 <= 4);
              }
              prodslot += 1;
            }
            while (prodslot <= 3);
          }
        }
      }
      if (this.resmode == 1 && simpleList2.Counter > -1)
      {
        simpleList2.Sort();
        let mut num21: i32 = -1;
        let mut num22: i32 = -1;
        let mut num23: i32 = 0;
        let mut index21: i32 = 1;
        for (let mut counter1: i32 = simpleList2.Counter; counter1 >= 0; counter1 += -1)
        {
          num22 += 1;
          num23 += 1;
          if (num23 > this.butcount)
          {
            num23 = 1;
            index21 += 1;
          }
          if (num23 == 1)
            this.butdata[index21] = num22;
          if (num22 == this.detailnr)
            this.curbut = index21;
          let mut num24: i32 = 0;
          if (num22 >= this.detailnr)
          {
            num21 += 1;
            if (num21 * 50 + 30 + 50 + 30 + 20 <= this.curheight)
            {
              num24 = 1;
              let mut index22: i32 = simpleList2.Id[counter1];
              let mut num25: i32 = simpleList2.Weight[counter1];
              let mut num26: i32 = simpleList2.Data1[counter1];
              let mut num27: i32 = simpleList2.Data2[counter1];
              num6 = simpleList2.Data3[counter1];
              let mut num28: i32 = simpleList2.Data4[counter1];
              let mut num29: i32 = 50 + 50 * num21;
              SimpleList simpleList3 = SimpleList::new();
              let mut index23: i32 = 0;
              do
              {
                if (this.game.Data.LocObj[index22].Production[index23] > -1)
                {
                  float num30 = Conversion.Int(Conversions.ToSingle(Strings.Trim(Conversion.Str( this.game.Data.LocObj[index22].TempProdPredict[index23]))));
                  let mut index24: i32 = 0;
                  do
                  {
                    let mut index25: i32 = this.game.Data.LocObj[index22].Production[index23];
                    if (index25 > -1 && this.game.Data.ItemTypeObj[index25].RegimeSlotsCost[index24] > -1 && this.game.Data.ItemTypeObj[index25].RegimeSlotsCostQty[index24] > 0 &  num30 > 0.0)
                    {
                      let mut nr: i32 = simpleList3.FindNr(this.game.Data.ItemTypeObj[index25].RegimeSlotsCost[index24]);
                      if (nr == -1)
                      {
                        simpleList3.Add(this.game.Data.ItemTypeObj[index25].RegimeSlotsCost[index24],  Math.Round( ( this.game.Data.ItemTypeObj[index25].RegimeSlotsCostQty[index24] * num30)));
                      }
                      else
                      {
                        int[] weight = simpleList3.Weight;
                        int[] numArray8 = weight;
                        let mut index26: i32 = nr;
                        let mut index27: i32 = index26;
                        let mut num31: i32 =  Math.Round( ( weight[index26] +  this.game.Data.ItemTypeObj[index25].RegimeSlotsCostQty[index24] * num30));
                        numArray8[index27] = num31;
                      }
                    }
                    index24 += 1;
                  }
                  while (index24 <= 4);
                }
                index23 += 1;
              }
              while (index23 <= 3);
              simpleList3.ReverseSort();
              let mut num32: i32 = -1;
              let mut counter2: i32 = simpleList3.Counter;
              for (let mut index28: i32 = 0; index28 <= counter2; index28 += 1)
              {
                if (index28 <= 3)
                {
                  tstring: String = Strings.Trim(Strings.Left(this.game.Data.RegimeSlotName[simpleList3.Id[index28]], 3));
                  float Number =  simpleList3.Weight[index28];
                  num32 += 1;
                  DrawMod.DrawTextVic2( graphics, tstring, this.game.VicFont3, num5 + 455 + num32 * 50, num29 + 12, this.game.VicColor2, this.game.VicColor2Shade);
                  DrawMod.DrawTextVic2( graphics, Strings.Trim(Conversion.Str( Number)), this.game.VicFont3, num5 + 455 + num32 * 50, num29 + 27, this.game.VicColor2, this.game.VicColor2Shade);
                }
              }
            }
          }
        }
      }
      if (num1 != 1)
        ;
      if (num2 != 1)
        ;
      num33: i32;
      if (Conversion.Int(1.0 +  simpleList2.Counter /  this.butcount) > 1.0)
      {
        DrawMod.DrawTextVic( graphics, "PAGE", this.game.VicFont2, 684 + num4, 30, this.game.VicColor1, this.game.VicColor1Shade);
        let mut num34: i32 =  Math.Round(Conversion.Int(1.0 +  simpleList2.Counter /  this.butcount));
        for (let mut index29: i32 = 1; index29 <= num34; index29 += 1)
        {
          if (this.curbut == index29)
          {
            int[] butpage = this.butpage;
            let mut index30: i32 = index29;
            let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONFLAGGED);
            let mut num35: i32 = this.AddSubPart( tsubpart, 691 + num4, 50 + (index29 - 1) * 18, 32, 16, 1);
            butpage[index30] = num35;
          }
          else
          {
            int[] butpage = this.butpage;
            let mut index31: i32 = index29;
            let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONUNFLAGGED);
            let mut num36: i32 = this.AddSubPart( tsubpart, 691 + num4, 50 + (index29 - 1) * 18, 32, 16, 1);
            butpage[index31] = num36;
          }
          num33 = 50 + (index29 - 1) * 18;
        }
      }
      DrawMod.DrawTextVic( graphics, "RES.", this.game.VicFont2, 689 + num4, num33 + 34, this.game.VicColor1, this.game.VicColor1Shade);
      if (this.resmode == 1)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONFLAGGED);
        this.butres = this.AddSubPart( tsubpart, 691 + num4, num33 + 54, 32, 16, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONUNFLAGGED);
        this.butres = this.AddSubPart( tsubpart, 691 + num4, num33 + 54, 32, 16, 1);
      }
      SimpleList simpleList4 = SimpleList::new();
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut tid: i32 = 0; tid <= unitCounter; tid += 1)
      {
        if (this.game.Data.UnitObj[tid].Regime == this.game.Data.Turn && this.game.Data.UnitObj[tid].IsHQ && this.game.Data.UnitObj[tid].PreDef == -1)
        {
          let mut num37: i32 = 0;
          let mut mapWidth: i32 = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
          for (let mut index32: i32 = 0; index32 <= mapWidth; index32 += 1)
          {
            let mut mapHeight: i32 = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
            for (let mut index33: i32 = 0; index33 <= mapHeight; index33 += 1)
            {
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index32, index33].Regime == this.game.Data.Turn && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index32, index33].Location > -1 && this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index32, index33].Location].HQ == tid)
                num37 = 1;
            }
          }
          if (num37 == 1)
            simpleList4.Add(tid, 0);
        }
      }
      this.OptionsListObj = ATListClass::new();
      let mut num38: i32 = 0;
      let mut tlistselect: i32 = 0;
      this.OptionsListObj.add("All HQs", -1);
      let mut counter3: i32 = simpleList4.Counter;
      for (let mut index34: i32 = 0; index34 <= counter3; index34 += 1)
      {
        num38 += 1;
        if (simpleList4.Id[index34] == this.detailnr2)
          tlistselect = num38;
        this.OptionsListObj.add(this.game.Data.UnitObj[simpleList4.Id[index34]].Name, simpleList4.Id[index34]);
      }
      this.OptionsListObj.add("No HQs", -2);
      let mut num39: i32 = num38 + 1;
      if (this.detailnr2 == -2)
        tlistselect = num39;
      if (this.OptionsListId > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, tlistselect);
        this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
      }
      else
      {
        let mut tsubpart: SubPartClass =  new ATListSubPartClass(this.OptionsListObj, 8, 280, tlistselect, this.game, tHeader: "Headquarters", tbackbitmap: ( this.OwnBitmap), bbx: (this.OwnBitmap.Width - 290 - num4), bby: 42);
        this.OptionsListId = this.AddSubPart( tsubpart, this.OwnBitmap.Width - 290 - num4, 42, 280, 176, 0);
      }
      this.OptionsList2Obj = ATListClass::new();
      if (this.resmode == 0)
      {
        if (this.detailnr2 > -1)
          this.OptionsList2Obj.add("Last Supply Req.", -1, Conversion.Str( this.game.Data.UnitObj[this.detailnr2].SupplyReq));
        let mut itemTypeCounter: i32 = this.game.Data.ItemTypeCounter;
        for (let mut index35: i32 = 0; index35 <= itemTypeCounter; index35 += 1)
        {
          if ( numArray1[index35] > 0.0)
            this.OptionsList2Obj.add(this.game.Data.ItemTypeObj[index35].Name, -1, Conversion.Str( numArray1[index35]));
        }
      }
      else
      {
        let mut counter4: i32 = simpleList1.Counter;
        for (let mut index36: i32 = 0; index36 <= counter4; index36 += 1)
        {
          if (simpleList1.Weight[index36] > 0)
            this.OptionsList2Obj.add(this.game.Data.RegimeSlotName[simpleList1.Id[index36]], -1, Conversion.Str( simpleList1.Weight[index36]));
        }
      }
      if (this.OptionsList2Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList2Id)].Refresh(this.OptionsList2Obj, -1);
        this.SubPartFlag[this.SubpartNr(this.OptionsList2Id)] = true;
      }
      else
      {
        let mut tsubpart: SubPartClass =  new ATListSubPartClass(this.OptionsList2Obj, 12, 280, -1, this.game, tHeader: "Total Production", tShowPair: true, tValueWidth: 80, tbackbitmap: ( this.OwnBitmap), bbx: (this.OwnBitmap.Width - 290 - num4), bby: 220);
        this.OptionsList2Id = this.AddSubPart( tsubpart, this.OwnBitmap.Width - 290 - num4, 220, 280, 224, 0);
      }
      if (Information.IsNothing( graphics))
        return;
      graphics.Dispose();
      graphics = (Graphics) null;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut mzcount: i32 = this.mzcount;
      for (let mut index: i32 = 0; index <= mzcount; index += 1)
      {
        if (x >= this.mzx[index] & x <= this.mzx2[index] && y >= this.mzy[index] & y <= this.mzy2[index])
        {
          this.game.EditObj.OrderLoc = this.mznr[index];
          this.game.SelectX = this.game.Data.LocObj[this.mznr[index]].X;
          this.game.SelectY = this.game.Data.LocObj[this.mznr[index]].Y;
          this.game.EditObj.MapSelected = this.game.Data.LocObj[this.mznr[index]].Map;
          let mut num1: i32 = 265;
          let mut selectX: i32 = this.game.SelectX;
          let mut selectY: i32 = this.game.SelectY;
          this.game.CornerX =  Math.Round( selectX - ( this.game.ScreenWidth / 2.0 - 0.0) / 53.0);
          this.game.CornerY =  Math.Round( selectY - ( this.game.ScreenHeight / 2.0 -  num1) / 48.0);
          if (this.game.CornerX < 0)
            this.game.CornerX = 0;
          if (this.game.CornerY < 0)
            this.game.CornerY = 0;
          if (this.game.Data.Round == 0)
            num1 += 100;
          let mut num2: i32 =  Math.Round( (this.game.ScreenWidth - 220 - 0) / 53.0);
          let mut num3: i32 =  Math.Round( (this.game.ScreenHeight - num1) / 48.0);
          let mut num4: i32 = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth - this.game.CornerX;
          let mut num5: i32 = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight - this.game.CornerY;
          if (num2 > num4)
            this.game.CornerX = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth - num2 + 2;
          if (num3 > num5)
            this.game.CornerY = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight - num3 + 2;
          if ((this.game.CornerX + 10) % 2 > 0)
            this += 1.game.CornerX;
          if ((this.game.CornerY + 10) % 2 > 0)
            this += 1.game.CornerY;
          this.game.SelectX = selectX;
          this.game.SelectY = selectY;
          this.dostuff();
          windowReturnClass.AddCommand(4, 44);
          windowReturnClass.AddCommand(4, 25);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = this.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num: i32 = this.SubPartID[index1];
            if (num == this.butdown)
            {
              this += 1.detailnr;
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.butres)
            {
              this.resmode = this.resmode != 0 ? 0 : 1;
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.butup)
            {
              --this.detailnr;
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.OptionsListId)
            {
              this.detailnr2 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.detailnr = 0;
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.OptionsList2Id)
            {
              this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            let mut index2: i32 = 0;
            while (this.butpage[index2] != this.SubPartID[index1])
            {
              index2 += 1;
              if (index2 > 999)
                return windowReturnClass;
            }
            this.detailnr = this.butdata[index2];
            this.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
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
