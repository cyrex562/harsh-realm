// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.BattlegroupWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class BattlegroupWindowClass : WindowClass
  {
     int okid;
     int cancelid;
     int transferid;
     int transfer2id;
     int oktextid;
     int sourceUnr;
     int targetUnr;
     SimpleList TL;
     int Pic1Id;
     int[] SlidersId;
     int[] SlidersSfnr;
     int SourceNextId;
     int SourcePrevId;
     int TargetNextId;
     int TargetPrevId;
     int SourcePage;
     int TargetPage;
     int SourcePageMax;
     int TargetPageMax;

    pub BattlegroupWindowClass(ref tGame: GameClass)
      : base(ref tGame, 1000, 760, 8)
    {
      this.SlidersId = new int[20];
      this.SlidersSfnr = new int[20];
      this.TL = SimpleList::new();
      this.sourceUnr = this.game.EditObj.UnitSelected;
      this.targetUnr = -1;
      this.SetUnits();
      this.View();
    }

    pub fn SetUnits()
    {
      let mut index: i32 =  0;
      do
      {
        if (this.SlidersId[index] > 0)
        {
          this.RemoveSubPart(this.SlidersId[index]);
          this.SlidersId[index] = 0;
        }
        index += 1;
      }
      while (index <= 19);
    }

    pub fn HandleToolTip(int x, int y)
    {
      base.HandleToolTip(x, y);
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            this.SubPartList[index].DescriptInfo(x - this.SubPartX[index], y - this.SubPartY[index]);
            if (Operators.CompareString(this.SubPartList[index].Descript, "", false) > 0)
            {
              this.game.EditObj.TipButton = true;
              this.game.EditObj.TipTitle = "";
              this.game.EditObj.TipText = this.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] > 0)
            this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = this.MouseTitle[index];
          this.game.EditObj.TipText = this.MouseText[index];
          break;
        }
      }
    }

    pub fn View()
    {
      let mut selectX: i32 =  this.game.SelectX;
      let mut selectY: i32 =  this.game.SelectY;
      if (this.cancelid > 0)
      {
        this.RemoveSubPart(this.cancelid);
        this.cancelid = 0;
      }
      if (this.transferid > 0)
      {
        this.RemoveSubPart(this.transferid);
        this.transferid = 0;
      }
      if (this.transfer2id > 0)
      {
        this.RemoveSubPart(this.transfer2id);
        this.transfer2id = 0;
      }
      if (this.SourceNextId > 0)
      {
        this.RemoveSubPart(this.SourceNextId);
        this.SourceNextId = 0;
      }
      if (this.SourcePrevId > 0)
      {
        this.RemoveSubPart(this.SourcePrevId);
        this.SourcePrevId = 0;
      }
      if (this.TargetNextId > 0)
      {
        this.RemoveSubPart(this.TargetNextId);
        this.TargetNextId = 0;
      }
      if (this.TargetPrevId > 0)
      {
        this.RemoveSubPart(this.TargetPrevId);
        this.TargetPrevId = 0;
      }
      this.TL.removeWeight0orLower();
      color: Color = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      this.ClearMouse();
      this.NewBackGroundAndClearAll(1000, 760, -1);
      Graphics toG = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawMessFrame(ref this.OwnBitmap, ref toG, 0, 0, 1000, 760);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      let mut battlegroupTemplate: i32 =  this.game.HandyFunctionsObj.GetBattlegroupTemplate(this.game.Data.Turn);
      let mut index1: i32 =  -1;
      if ( this.game.Data.RuleVar[486] > 0.0)
        index1 = this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[486]));
      let mut x1_1: i32 =  5;
      let mut num1: i32 =  980;
      let mut h1: i32 =  110;
      let mut y1_1: i32 =  10;
      DrawMod.DrawBlockGradient2(ref toG, x1_1, y1_1, 235, 40, this.game.MarcCol1, this.game.MarcCol2);
      DrawMod.DrawBlockGradient2(ref toG, x1_1 + 240, y1_1, 235, h1, this.game.MarcCol1, this.game.MarcCol2);
      DrawMod.DrawBlockGradient2(ref toG, x1_1 + 480, y1_1, (int) byte.MaxValue, h1, this.game.MarcCol1, this.game.MarcCol2);
      DrawMod.DrawBlockGradient2(ref toG, x1_1 + 740, y1_1, 235, 40, this.game.MarcCol1, this.game.MarcCol2);
      tstring1: String = "Source unit list";
      DrawMod.DrawTextColouredMarcCenter(ref toG, tstring1, this.game.MarcFont3, x1_1 + 120, y1_1 + 10, Color.White);
      tstring2: String = "Target unit list";
      DrawMod.DrawTextColouredMarcCenter(ref toG, tstring2, this.game.MarcFont3, x1_1 + 840, y1_1 + 10, Color.White);
      let mut num2: i32 =  740;
      let mut num3: i32 =  5;
      let mut num4: i32 =  52;
      let mut num5: i32 =  235;
      let mut num6: i32 =  37;
      SimpleList simpleList1 = SimpleList::new();
      let mut unitCounter1: i32 =  this.game.Data.MapObj[0].HexObj[selectX, selectY].UnitCounter;
      for (let mut index2: i32 =  0; index2 <= unitCounter1; index2 += 1)
      {
        let mut unit: i32 =  this.game.Data.MapObj[0].HexObj[selectX, selectY].UnitList[index2];
        if (this.sourceUnr == -1)
          this.sourceUnr = unit;
        simpleList1.Add(unit, 1);
      }
      let mut counter1: i32 =  simpleList1.Counter;
      Rectangle trect1;
      for (let mut index3: i32 =  0; index3 <= counter1; index3 += 1)
      {
        let mut index4: i32 =  simpleList1.Id[index3];
        if (index4 == this.sourceUnr)
        {
          DrawMod.DrawBlock(ref toG, num3, num4, num5, num6, 0, 0, 0, 64);
          DrawMod.DrawRectangle(ref toG, num3, num4 - 1, num5, num6 + 2, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
        }
        else if (index4 == this.targetUnr)
          DrawMod.DrawBlock(ref toG, num3, num4, num5, num6, 128, 0, 0, 128);
        else
          DrawMod.DrawBlock(ref toG, num3, num4, num5, num6, 0, 0, 0, 128);
        this.game.CustomBitmapObj.DrawUnit(index4, toG: toG, tx: (num3 + 8), ty: num4);
        str: String = this.game.Data.UnitObj[index4].Name;
        if (str.Length > 25)
          str = Strings.Left(str, 25) + ".";
        DrawMod.DrawTextColouredMarc(ref toG, str, this.game.MarcFont4, num3 + 50, num4 + 5, Color.White);
        let mut hq: i32 =  this.game.Data.UnitObj[index4].HQ;
        tstring3: String = hq <= -1 ? "no HQ" : "HQ: " + this.game.Data.UnitObj[hq].Name;
        DrawMod.DrawTextColouredMarc(ref toG, tstring3, this.game.MarcFont5, num3 + 52, num4 + 22, Color.LightGray);
        if (index4 != this.targetUnr)
        {
          trect1 = Rectangle::new(num3, num4, num5, num6);
          this.AddMouse(ref trect1, "", "", 1, index4);
        }
        num4 += 39;
      }
      let mut num7: i32 =  745;
      let mut num8: i32 =  52;
      let mut num9: i32 =  235;
      let mut num10: i32 =  37;
      SimpleList simpleList2 = SimpleList::new();
      let mut unitCounter2: i32 =  this.game.Data.MapObj[0].HexObj[selectX, selectY].UnitCounter;
      for (let mut index5: i32 =  0; index5 <= unitCounter2; index5 += 1)
      {
        let mut unit: i32 =  this.game.Data.MapObj[0].HexObj[selectX, selectY].UnitList[index5];
        simpleList2.Add(unit, 1);
      }
      if ( this.game.Data.RuleVar[486] > 0.0)
      {
        if (-3 == this.targetUnr)
        {
          DrawMod.DrawBlock(ref toG, num7, num8, num9, num10, 0, 0, 0, 64);
          DrawMod.DrawRectangle(ref toG, num7, num8 - 1, num9, num10 + 2, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
        }
        else
          DrawMod.DrawBlock(ref toG, num7, num8, num9, num10, 0, 0, 0, 128);
        tstring4: String = "Scrap & Scuttle";
        DrawMod.DrawTextColouredMarc(ref toG, tstring4, this.game.MarcFont4, num7 + 50, num8 + 5, Color.White);
        tstring5: String = "Destroys selected equipment";
        DrawMod.DrawTextColouredMarc(ref toG, tstring5, this.game.MarcFont5, num7 + 52, num8 + 22, Color.LightGray);
        trect1 = Rectangle::new(num7, num8, num9, num10);
        let mut trect2: &Rectangle = &trect1
        this.AddMouse(ref trect2, "", "", 2, -3);
        num8 += 39;
      }
      if (this.game.HandyFunctionsObj.GetBattlegroupTemplate(this.game.Data.Turn) > -1)
      {
        if (-2 == this.targetUnr)
        {
          DrawMod.DrawBlock(ref toG, num7, num8, num9, num10, 0, 0, 0, 64);
          DrawMod.DrawRectangle(ref toG, num7, num8 - 1, num9, num10 + 2, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
        }
        else
          DrawMod.DrawBlock(ref toG, num7, num8, num9, num10, 0, 0, 0, 128);
        tstring6: String = "New Battlegroup";
        if (battlegroupTemplate > -1)
        {
          tstring6 = "New " + this.game.Data.HistoricalUnitObj[battlegroupTemplate].Name;
          if (this.game.Data.HistoricalUnitObj[battlegroupTemplate].PP > 0)
            tstring6 = tstring6 + " [" + this.game.Data.HistoricalUnitObj[battlegroupTemplate].PP.ToString() + "pp]";
        }
        DrawMod.DrawTextColouredMarc(ref toG, tstring6, this.game.MarcFont4, num7 + 50, num8 + 5, Color.White);
        tstring7: String = "Newly created Battlegroup";
        DrawMod.DrawTextColouredMarc(ref toG, tstring7, this.game.MarcFont5, num7 + 52, num8 + 22, Color.LightGray);
        trect1 = Rectangle::new(num7, num8, num9, num10);
        let mut trect3: &Rectangle = &trect1
        this.AddMouse(ref trect3, "", "", 2, -2);
        num8 += 39;
      }
      let mut counter2: i32 =  simpleList2.Counter;
      for (let mut index6: i32 =  0; index6 <= counter2; index6 += 1)
      {
        let mut index7: i32 =  simpleList2.Id[index6];
        if (index7 == this.targetUnr)
        {
          DrawMod.DrawBlock(ref toG, num7, num8, num9, num10, 0, 0, 0, 64);
          DrawMod.DrawRectangle(ref toG, num7, num8 - 1, num9, num10 + 2, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
        }
        else if (index7 == this.sourceUnr)
          DrawMod.DrawBlock(ref toG, num7, num8, num9, num10, 128, 0, 0, 128);
        else
          DrawMod.DrawBlock(ref toG, num7, num8, num9, num10, 0, 0, 0, 128);
        this.game.CustomBitmapObj.DrawUnit(index7, toG: toG, tx: (num7 + 8), ty: num8);
        str: String = this.game.Data.UnitObj[index7].Name;
        if (str.Length > 25)
          str = Strings.Left(str, 25) + ".";
        DrawMod.DrawTextColouredMarc(ref toG, str, this.game.MarcFont4, num7 + 50, num8 + 5, Color.White);
        let mut hq: i32 =  this.game.Data.UnitObj[index7].HQ;
        tstring8: String = hq <= -1 ? "no HQ" : "HQ: " + this.game.Data.UnitObj[hq].Name;
        DrawMod.DrawTextColouredMarc(ref toG, tstring8, this.game.MarcFont5, num7 + 52, num8 + 22, Color.LightGray);
        if (index7 != this.sourceUnr)
        {
          trect1 = Rectangle::new(num7, num8, num9, num10);
          let mut trect4: &Rectangle = &trect1
          this.AddMouse(ref trect4, "", "", 2, index7);
        }
        num8 += 39;
      }
      let mut num11: i32 =  11;
      let mut x1_2: i32 =  485;
      let mut y1_2: i32 =  125;
      let mut maxValue: i32 =  (int) byte.MaxValue;
      let mut h2: i32 =  40;
      SimpleList simpleList3 = this.TL.Clone();
      SimpleList simpleList4 = SimpleList::new();
      SimpleList SL = SimpleList::new();
      let mut index8: i32 =  -1;
      let mut num12: i32 =  -1;
      if (this.targetUnr > -1)
      {
        index8 = this.game.Data.UnitObj[this.targetUnr].Historical;
        let mut historicalSubPart: i32 =  this.game.Data.UnitObj[this.targetUnr].HistoricalSubPart;
        if (index8 > -1)
        {
          let mut subPart: i32 =  this.game.Data.HistoricalUnitObj[index8].SubParts[historicalSubPart];
          if (subPart > 0)
          {
            let mut preDef: i32 =  this.game.HandyFunctionsObj.GetPreDef(subPart);
            let mut sfCount: i32 =  this.game.Data.UnitObj[preDef].SFCount;
            for (let mut index9: i32 =  0; index9 <= sfCount; index9 += 1)
            {
              let mut sf: i32 =  this.game.Data.UnitObj[preDef].SFList[index9];
              let mut type: i32 =  this.game.Data.SFObj[sf].Type;
              let mut people: i32 =  this.game.Data.SFObj[sf].People;
              let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
              let mut reinforcementType: i32 =  this.game.Data.SFTypeObj[type].ReinforcementType;
              simpleList4.AddWeight(reinforcementType, qty, people);
            }
          }
        }
        let mut sfCount1: i32 =  this.game.Data.UnitObj[this.targetUnr].SFCount;
        for (let mut index10: i32 =  0; index10 <= sfCount1; index10 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[this.targetUnr].SFList[index10];
          let mut type: i32 =  this.game.Data.SFObj[sf].Type;
          let mut people: i32 =  this.game.Data.SFObj[sf].People;
          let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
          let mut reinforcementType: i32 =  this.game.Data.SFTypeObj[type].ReinforcementType;
          SL.AddWeight(reinforcementType, qty, people);
        }
      }
      let mut counter3: i32 =  simpleList3.Counter;
      int qty1;
      for (let mut index11: i32 =  0; index11 <= counter3; index11 += 1)
      {
        let mut index12: i32 =  simpleList3.Id[index11];
        let mut type: i32 =  this.game.Data.SFObj[index12].Type;
        let mut people: i32 =  this.game.Data.SFObj[index12].People;
        qty1 = this.game.Data.SFObj[index12].Qty;
        let mut reinforcementType: i32 =  this.game.Data.SFTypeObj[type].ReinforcementType;
        SL.AddWeight(reinforcementType, simpleList3.Weight[index11], people);
      }
      SimpleList simpleList5 = simpleList4.Clone();
      simpleList5.RemoveWeight(ref SL);
      SimpleList simpleList6 = simpleList3.Clone();
      if (this.targetUnr > -1)
      {
        let mut sfCount: i32 =  this.game.Data.UnitObj[this.targetUnr].SFCount;
        for (let mut index13: i32 =  0; index13 <= sfCount; index13 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[this.targetUnr].SFList[index13];
          let mut type1: i32 =  this.game.Data.SFObj[sf].Type;
          let mut reinforcementType: i32 =  this.game.Data.SFTypeObj[type1].ReinforcementType;
          let mut people1: i32 =  this.game.Data.SFObj[sf].People;
          for (let mut counter4: i32 =  simpleList3.Counter; counter4 >= 0; counter4 += -1)
          {
            let mut index14: i32 =  simpleList3.Id[counter4];
            let mut type2: i32 =  this.game.Data.SFObj[index14].Type;
            let mut people2: i32 =  this.game.Data.SFObj[index14].People;
            if (type2 == type1 & people2 == people1)
              simpleList6.RemoveNr(counter4);
          }
        }
      }
      this.TargetPageMax = this.targetUnr <= -1 ? Convert.ToInt32(Decimal.Divide(Math.Ceiling(new Decimal(0 + simpleList6.Counter + 1)), new Decimal(num11))) : (int) Math.Round(Math.Ceiling( (this.game.Data.UnitObj[this.targetUnr].SFCount + 1 + simpleList6.Counter + 1) /  num11));
      if (this.TargetPageMax < 1)
        this.TargetPageMax = 1;
      if (this.TargetPage < 1)
        this.TargetPage = 1;
      if (this.TargetPage > this.TargetPageMax)
        this.TargetPage = this.TargetPageMax;
      int num13;
      int num14;
      int num15;
      int num16;
      int num17;
      if (this.targetUnr > -1)
      {
        let mut sfCount: i32 =  this.game.Data.UnitObj[this.targetUnr].SFCount;
        for (let mut index15: i32 =  0; index15 <= sfCount; index15 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[this.targetUnr].SFList[index15];
          let mut type3: i32 =  this.game.Data.SFObj[sf].Type;
          let mut reinforcementType: i32 =  this.game.Data.SFTypeObj[type3].ReinforcementType;
          let mut people3: i32 =  this.game.Data.SFObj[sf].People;
          let mut qty2: i32 =  this.game.Data.SFObj[sf].Qty;
          let mut num18: i32 =  0;
          for (let mut counter5: i32 =  simpleList3.Counter; counter5 >= 0; counter5 += -1)
          {
            let mut index16: i32 =  simpleList3.Id[counter5];
            let mut type4: i32 =  this.game.Data.SFObj[index16].Type;
            let mut people4: i32 =  this.game.Data.SFObj[index16].People;
            let mut qty3: i32 =  this.game.Data.SFObj[index16].Qty;
            if (type4 == type3 & people4 == people3)
            {
              num18 += simpleList3.Weight[counter5];
              simpleList3.RemoveNr(counter5);
            }
          }
          if (num18 <= 0)
            num18 = 0;
          if (this.game.Data.SFTypeObj[type3].CarryCap < 1 & this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type3].MoveType] == 2)
            num13 += this.game.Data.SFTypeObj[type3].Weight * (qty2 + num18);
          if (this.game.Data.SFTypeObj[type3].CarryCap < 1 & this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type3].MoveType] == 2)
            num14 += this.game.Data.SFTypeObj[type3].manpower * (qty2 + num18);
          if (this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type3].MoveType] != 2)
            num15 += this.game.Data.SFTypeObj[type3].CarryCap * (qty2 + num18);
          if (this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type3].MoveType] != 2)
            num16 += this.game.Data.SFTypeObj[type3].manpowerCarry * (qty2 + num18);
          num17 += this.game.Data.SFTypeObj[type3].PowerPts * (qty2 - num18);
          if (index15 >= (this.TargetPage - 1) * num11 & index15 <= (this.TargetPage - 1) * num11 + (num11 - 1))
          {
            DrawMod.DrawBlock(ref toG, x1_2, y1_2, maxValue, h2, 0, 0, 0, 128);
            num12 += 1;
            let mut people5: i32 =  this.game.Data.SFObj[sf].People;
            let mut symbolSpriteId: i32 =  this.game.Data.SFTypeObj[type3].SymbolSpriteID;
            ref Graphics local1 = ref toG;
            bitmap: Bitmap = BitmapStore.GetBitmap(symbolSpriteId);
            ref local2: Bitmap = ref bitmap;
            let mut sftypenr: i32 =  type3;
            let mut ppl: i32 =  people5;
            let mut tx: i32 =  x1_2 + 3;
            let mut ty: i32 =  y1_2 + 7;
            DrawMod.DrawWithArtCode(ref local1, ref local2, 34, 17, sftypenr, ppl, tx, ty, -1, -1);
            let mut num19: i32 =  qty2 * this.game.Data.SFTypeObj[type3].Ratio;
            let mut num20: i32 =  num18 * this.game.Data.SFTypeObj[type3].Ratio;
            tstring9: String = (num19 + num20).ToString() + "x " + this.game.Data.SFTypeObj[type3].Name + "(" + Strings.Left(this.game.Data.PeopleObj[people5].Name, 3) + ")";
            DrawMod.DrawTextColouredMarc(ref toG, tstring9, this.game.MarcFont4, x1_2 + 65, y1_2 + 2, Color.White);
            if (this.targetUnr == -3)
            {
              tstring10: String = this.game.Data.SFTypeObj[type3].scrapable.ToString() + "%";
              DrawMod.DrawTextColouredMarc(ref toG, tstring10, this.game.MarcFont4, x1_2 + maxValue - 40, y1_2 + 15, Color.White);
            }
            else if (num20 > 0)
            {
              tstring11: String = "+" + num20.ToString();
              DrawMod.DrawTextColouredMarc(ref toG, tstring11, this.game.MarcFont4, x1_2 + maxValue - 40, y1_2 + 15, Color.White);
            }
            if (reinforcementType > -1 & index8 > -1 & this.targetUnr > -1)
            {
              let mut num21: i32 =  SL.FindWeight(reinforcementType, people5) * this.game.Data.ReinfRatio[reinforcementType];
              let mut num22: i32 =  simpleList4.FindWeight(reinforcementType, people5) * this.game.Data.ReinfRatio[reinforcementType];
              let mut num23: i32 =  0;
              if (this.targetUnr > -1 && this.game.Data.UnitObj[this.targetUnr].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.targetUnr].Historical].Type == 8)
                num23 = 1;
              if ((num21 > 0 | num22 > 0) & !this.game.HandyFunctionsObj.CheckIsBattlegroup(this.targetUnr) & num23 == 0)
              {
                tstring12: String = "Reinf. max: " + num21.ToString() + "/" + num22.ToString();
                if (num21 > num22)
                  DrawMod.DrawTextColouredMarc(ref toG, tstring12, this.game.MarcFont4, x1_2 + 65, y1_2 + 20, Color.Red);
                else
                  DrawMod.DrawTextColouredMarc(ref toG, tstring12, this.game.MarcFont4, x1_2 + 65, y1_2 + 20, Color.Gray);
              }
              else
              {
                tstring13: String = "No organisational limit";
                DrawMod.DrawTextColouredMarc(ref toG, tstring13, this.game.MarcFont4, x1_2 + 65, y1_2 + 20, Color.Gray);
              }
            }
            else
            {
              tstring14: String = "No organisational limit";
              DrawMod.DrawTextColouredMarc(ref toG, tstring14, this.game.MarcFont4, x1_2 + 65, y1_2 + 20, Color.Gray);
            }
            y1_2 += 42;
          }
        }
      }
      let mut num24: i32 =  num12;
      int num25;
      if (this.targetUnr == -2 | this.targetUnr == -3 | this.targetUnr > -1)
      {
        let mut counter6: i32 =  simpleList3.Counter;
        for (let mut index17: i32 =  0; index17 <= counter6; index17 += 1)
        {
          let mut index18: i32 =  simpleList3.Id[index17];
          let mut type: i32 =  this.game.Data.SFObj[index18].Type;
          let mut reinforcementType: i32 =  this.game.Data.SFTypeObj[type].ReinforcementType;
          let mut people: i32 =  this.game.Data.SFObj[index18].People;
          qty1 = this.game.Data.SFObj[index18].Qty;
          let mut num26: i32 =  simpleList3.Weight[index17];
          if (num26 <= 0)
            num26 = 0;
          if (this.game.Data.SFTypeObj[type].CarryCap < 1 & this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type].MoveType] == 2)
            num13 += this.game.Data.SFTypeObj[type].Weight * num26;
          if (this.game.Data.SFTypeObj[type].CarryCap < 1 & this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type].MoveType] == 2)
            num14 += this.game.Data.SFTypeObj[type].manpower * num26;
          if (this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type].MoveType] != 2)
            num15 += this.game.Data.SFTypeObj[type].CarryCap * num26;
          if (this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type].MoveType] != 2)
            num16 += this.game.Data.SFTypeObj[type].manpowerCarry * num26;
          num17 += this.game.Data.SFTypeObj[type].PowerPts * num26;
          if (index17 + num24 + 1 >= (this.TargetPage - 1) * num11 & index17 + num24 + 1 <= (this.TargetPage - 1) * num11 + (num11 - 1))
          {
            DrawMod.DrawBlock(ref toG, x1_2, y1_2, maxValue, h2, 0, 0, 0, 128);
            num12 += 1;
            let mut symbolSpriteId: i32 =  this.game.Data.SFTypeObj[type].SymbolSpriteID;
            ref Graphics local3 = ref toG;
            bitmap: Bitmap = BitmapStore.GetBitmap(symbolSpriteId);
            ref local4: Bitmap = ref bitmap;
            let mut sftypenr: i32 =  type;
            let mut ppl: i32 =  people;
            let mut tx: i32 =  x1_2 + 3;
            let mut ty: i32 =  y1_2 + 7;
            DrawMod.DrawWithArtCode(ref local3, ref local4, 34, 17, sftypenr, ppl, tx, ty, -1, -1);
            let mut num27: i32 =  0;
            let mut num28: i32 =  num26 * this.game.Data.SFTypeObj[type].Ratio;
            strArray1: Vec<String> = new string[6];
            strArray2: Vec<String> = strArray1;
            num25 = num27 + num28;
            str: String = num25.ToString();
            strArray2[0] = str;
            strArray1[1] = "x ";
            strArray1[2] = this.game.Data.SFTypeObj[type].Name;
            strArray1[3] = "(";
            strArray1[4] = Strings.Left(this.game.Data.PeopleObj[people].Name, 3);
            strArray1[5] = ")";
            tstring15: String = string.Concat(strArray1);
            DrawMod.DrawTextColouredMarc(ref toG, tstring15, this.game.MarcFont4, x1_2 + 65, y1_2 + 2, Color.White);
            if (this.targetUnr == -3)
            {
              tstring16: String = this.game.Data.SFTypeObj[type].scrapable.ToString() + "%";
              DrawMod.DrawTextColouredMarc(ref toG, tstring16, this.game.MarcFont4, x1_2 + maxValue - 40, y1_2 + 15, Color.White);
            }
            else if (num28 > 0)
            {
              tstring17: String = "+" + num28.ToString();
              DrawMod.DrawTextColouredMarc(ref toG, tstring17, this.game.MarcFont4, x1_2 + maxValue - 40, y1_2 + 15, Color.White);
            }
            if (this.targetUnr == -3)
            {
              tstring18: String = "Earmarked for scrapping";
              DrawMod.DrawTextColouredMarc(ref toG, tstring18, this.game.MarcFont4, x1_2 + 65, y1_2 + 20, Color.Gray);
            }
            else if (reinforcementType > -1 & index8 > -1 & this.targetUnr > -1)
            {
              let mut num29: i32 =  SL.FindWeight(reinforcementType, people) * this.game.Data.ReinfRatio[reinforcementType];
              let mut num30: i32 =  simpleList4.FindWeight(reinforcementType, people) * this.game.Data.ReinfRatio[reinforcementType];
              let mut num31: i32 =  0;
              if (this.targetUnr > -1 && this.game.Data.UnitObj[this.targetUnr].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.targetUnr].Historical].Type == 8)
                num31 = 1;
              if ((num29 > 0 | num30 > 0) & !this.game.HandyFunctionsObj.CheckIsBattlegroup(this.targetUnr) & num31 == 0)
              {
                tstring19: String = "Reinf. max: " + num29.ToString() + "/" + num30.ToString();
                if (num29 > num30)
                  DrawMod.DrawTextColouredMarc(ref toG, tstring19, this.game.MarcFont4, x1_2 + 65, y1_2 + 20, Color.Red);
                else
                  DrawMod.DrawTextColouredMarc(ref toG, tstring19, this.game.MarcFont4, x1_2 + 65, y1_2 + 20, Color.Gray);
              }
              else
              {
                tstring20: String = "No organisational limit";
                DrawMod.DrawTextColouredMarc(ref toG, tstring20, this.game.MarcFont4, x1_2 + 65, y1_2 + 20, Color.Gray);
              }
            }
            else
            {
              tstring21: String = "No organisational limit";
              DrawMod.DrawTextColouredMarc(ref toG, tstring21, this.game.MarcFont4, x1_2 + 65, y1_2 + 20, Color.Gray);
            }
            y1_2 += 42;
          }
        }
      }
      let mut y1_3: i32 =  125 + num11 * 42;
      DrawMod.DrawBlock(ref toG, x1_2, y1_3, maxValue, h2, 0, 0, 0, 128);
      if (this.TargetPage > 1)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(" < ", 50, tBackbitmap: (ref this.OwnBitmap), bbx: (x1_2 + 10), bby: (y1_3 + 4), usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.TargetPrevId = this.AddSubPart(ref tsubpart, x1_2 + 10, y1_3 + 4, 50, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(" < ", 50, tBackbitmap: (ref this.OwnBitmap), bbx: (x1_2 + 10), bby: (y1_3 + 4), tinactive: true, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.TargetPrevId = this.AddSubPart(ref tsubpart, x1_2 + 10, y1_3 + 4, 50, 35, 0);
      }
      if (this.TargetPage < this.TargetPageMax)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(" > ", 50, tBackbitmap: (ref this.OwnBitmap), bbx: (x1_2 + 200), bby: (y1_3 + 4), usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.TargetNextId = this.AddSubPart(ref tsubpart, x1_2 + 200, y1_3 + 4, 50, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(" > ", 50, tBackbitmap: (ref this.OwnBitmap), bbx: (x1_2 + 200), bby: (y1_3 + 4), tinactive: true, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.TargetNextId = this.AddSubPart(ref tsubpart, x1_2 + 200, y1_3 + 4, 50, 35, 0);
      }
      tstring22: String = this.TargetPage.ToString() + "/" + this.TargetPageMax.ToString();
      DrawMod.DrawTextColouredMarcCenter(ref toG, tstring22, this.game.MarcFont4, x1_2 + 130, y1_3 + 12, Color.White);
      let mut x1_3: i32 =  245;
      let mut y1_4: i32 =  125;
      let mut w1: i32 =  235;
      let mut h3: i32 =  40;
      this.SourcePageMax = this.sourceUnr <= -1 ? Convert.ToInt32(Decimal.Divide(Math.Ceiling(new Decimal(0 + simpleList6.Counter + 1)), new Decimal(num11))) : (int) Math.Round(Math.Ceiling( (this.game.Data.UnitObj[this.sourceUnr].SFCount + 1) /  num11));
      if (this.SourcePageMax < 1)
        this.SourcePageMax = 1;
      if (this.SourcePage < 1)
        this.SourcePage = 1;
      if (this.SourcePage > this.SourcePageMax)
        this.SourcePage = this.SourcePageMax;
      int num32;
      int num33;
      int num34;
      int num35;
      int num36;
      if (this.sourceUnr > -1)
      {
        let mut index19: i32 =  -1;
        let mut sfCount: i32 =  this.game.Data.UnitObj[this.sourceUnr].SFCount;
        for (let mut index20: i32 =  0; index20 <= sfCount; index20 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[this.sourceUnr].SFList[index20];
          let mut type: i32 =  this.game.Data.SFObj[sf].Type;
          let mut people6: i32 =  this.game.Data.SFObj[sf].People;
          let mut reinforcementType: i32 =  this.game.Data.SFTypeObj[type].ReinforcementType;
          let mut num37: i32 =  this.game.Data.SFObj[sf].Qty;
          let mut num38: i32 =  this.TL.FindWeight(sf);
          if (num38 <= 0)
            num38 = 0;
          if (num38 > 0)
            num37 = num37;
          let mut people7: i32 =  this.game.Data.SFObj[sf].People;
          if (this.game.Data.SFTypeObj[type].CarryCap < 1 & this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type].MoveType] == 2)
            num32 += this.game.Data.SFTypeObj[type].manpower * (num37 - num38);
          if (this.game.Data.SFTypeObj[type].CarryCap < 1 & this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type].MoveType] == 2)
            num33 += this.game.Data.SFTypeObj[type].Weight * (num37 - num38);
          if (this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type].MoveType] != 2)
            num34 += this.game.Data.SFTypeObj[type].CarryCap * (num37 - num38);
          if (this.game.Data.transportMovementType[this.game.Data.SFTypeObj[type].MoveType] != 2)
            num35 += this.game.Data.SFTypeObj[type].manpowerCarry * (num37 - num38);
          num36 += this.game.Data.SFTypeObj[type].PowerPts * (num37 - num38);
          if (index20 >= (this.SourcePage - 1) * num11 & index20 <= (this.SourcePage - 1) * num11 + (num11 - 1))
          {
            bool flag1 = true;
            if (reinforcementType > -1)
            {
              if (this.game.Data.Product == 7)
              {
                if (simpleList4.FindWeight(reinforcementType) <= 0)
                  flag1 = false;
              }
              else if (simpleList4.FindWeight(reinforcementType, people7) <= 0)
                flag1 = false;
            }
            else
              flag1 = false;
            if (this.targetUnr > -1)
            {
              if (this.game.HandyFunctionsObj.CheckIsBattlegroup(this.targetUnr))
                flag1 = true;
            }
            else
              flag1 = true;
            if (this.targetUnr == -3)
            {
              if (index1 > -1)
              {
                if (Conversions.ToInteger(this.game.Data.StringListObj[index1].GetData(0, people7, 1)) < 1)
                  flag1 = false;
                if (this.game.Data.SFTypeObj[type].scrapable <= 0)
                  flag1 = false;
              }
              else
                flag1 = false;
            }
            if (this.targetUnr > -1 && this.game.Data.UnitObj[this.targetUnr].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.targetUnr].Historical].Type == 8)
              flag1 = true;
            if (flag1)
              DrawMod.DrawBlock(ref toG, x1_3, y1_4, w1, h3, 0, 0, 0, 128);
            else
              DrawMod.DrawBlock(ref toG, x1_3, y1_4, w1, h3, 128, 0, 0, 128);
            index19 += 1;
            let mut symbolSpriteId: i32 =  this.game.Data.SFTypeObj[type].SymbolSpriteID;
            ref Graphics local5 = ref toG;
            bitmap: Bitmap = BitmapStore.GetBitmap(symbolSpriteId);
            ref local6: Bitmap = ref bitmap;
            let mut sftypenr: i32 =  type;
            let mut ppl: i32 =  people7;
            let mut tx: i32 =  x1_3 + 3;
            let mut ty: i32 =  y1_4 + 7;
            DrawMod.DrawWithArtCode(ref local5, ref local6, 34, 17, sftypenr, ppl, tx, ty, -1, -1);
            let mut num39: i32 =  num37 * this.game.Data.SFTypeObj[type].Ratio;
            tstring23: String = num39.ToString() + "x " + this.game.Data.SFTypeObj[type].Name + "(" + Strings.Left(this.game.Data.PeopleObj[people7].Name, 3) + ")";
            bool flag2 = false;
            bool flag3 = false;
            if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.sourceUnr].Historical].GiveHisVarValue(11) > 0)
            {
              if (this.targetUnr > -1)
              {
                if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.targetUnr].Historical].Type < 8 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.targetUnr].Historical].GiveHisVarValue(11) < 1)
                  flag2 = true;
              }
              else if (this.targetUnr == -2)
              {
                flag3 = true;
                flag2 = true;
              }
            }
            if (this.game.Data.PeopleObj[people7].tv1 >= 11 & this.game.Data.PeopleObj[people7].tv1 <= 19)
            {
              if (this.targetUnr > -1)
              {
                if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.targetUnr].Historical].Type < 8 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.targetUnr].Historical].GiveHisVarValue(11) < 1)
                  flag2 = true;
              }
              else if (this.targetUnr == -2)
              {
                flag3 = true;
                flag2 = true;
              }
            }
            if (flag2)
            {
              DrawMod.DrawTextColouredMarc(ref toG, tstring23, this.game.MarcFont4, x1_3 + 65, y1_4 + 2, Color.White);
              if (flag3)
                DrawMod.DrawTextColouredMarc(ref toG, "No Battlegroup for Militia", this.game.MarcFont4, x1_3 + 65, y1_4 + 20, Color.Gray);
              else
                DrawMod.DrawTextColouredMarc(ref toG, "Target unsuitable for Militia", this.game.MarcFont4, x1_3 + 65, y1_4 + 20, Color.Gray);
            }
            else if (this.game.Data.SFTypeObj[type].Theater == 2 && this.targetUnr == -2)
            {
              DrawMod.DrawTextColouredMarc(ref toG, "No Battlegroup for Air", this.game.MarcFont4, x1_3 + 65, y1_4 + 20, Color.Gray);
              flag2 = true;
            }
            if (!flag2)
            {
              if (flag1 | this.targetUnr == -2)
              {
                if (this.SlidersId[index19] < 1)
                {
                  int[] slidersId = this.SlidersId;
                  let mut index21: i32 =  index19;
                  let mut tsubpart: SubPartClass =  new NumberSliderSubPartClass2(this.game, "", " of " + num39.ToString() + "x " + this.game.Data.SFTypeObj[type].Name + "(" + Strings.Left(this.game.Data.PeopleObj[people7].Name, 3) + ")", 190, 0, num37 * this.game.Data.SFTypeObj[type].Ratio, num38 * this.game.Data.SFTypeObj[type].Ratio, tsmallchange: this.game.Data.SFTypeObj[type].Ratio, tbackbitmap: (ref this.OwnBitmap), bbx: (x1_3 + 45), bby: (y1_4 + 0));
                  let mut num40: i32 =  this.AddSubPart(ref tsubpart, x1_3 + 45, y1_4 + 0, 190, 40, 0);
                  slidersId[index21] = num40;
                  this.SlidersSfnr[index19] = sf;
                }
              }
              else if (this.targetUnr == -3)
              {
                DrawMod.DrawTextColouredMarc(ref toG, tstring23, this.game.MarcFont4, x1_3 + 65, y1_4 + 2, Color.White);
                DrawMod.DrawTextColouredMarc(ref toG, "Cannot be Scrapped", this.game.MarcFont4, x1_3 + 65, y1_4 + 20, Color.Gray);
              }
              else
              {
                DrawMod.DrawTextColouredMarc(ref toG, tstring23, this.game.MarcFont4, x1_3 + 65, y1_4 + 2, Color.White);
                DrawMod.DrawTextColouredMarc(ref toG, "No Target Unit TOE match", this.game.MarcFont4, x1_3 + 65, y1_4 + 20, Color.Gray);
              }
            }
            y1_4 += 42;
          }
        }
      }
      let mut y1_5: i32 =  125 + num11 * 42;
      DrawMod.DrawBlock(ref toG, x1_3, y1_5, w1, h3, 0, 0, 0, 128);
      if (this.SourcePage > 1)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(" < ", 50, tBackbitmap: (ref this.OwnBitmap), bbx: (x1_3 + 10), bby: (y1_5 + 4), usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.SourcePrevId = this.AddSubPart(ref tsubpart, x1_3 + 10, y1_5 + 4, 50, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(" < ", 50, tBackbitmap: (ref this.OwnBitmap), bbx: (x1_3 + 10), bby: (y1_5 + 4), tinactive: true, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.SourcePrevId = this.AddSubPart(ref tsubpart, x1_3 + 10, y1_5 + 4, 50, 35, 0);
      }
      if (this.SourcePage < this.SourcePageMax)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(" > ", 50, tBackbitmap: (ref this.OwnBitmap), bbx: (x1_3 + 180), bby: (y1_5 + 4), usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.SourceNextId = this.AddSubPart(ref tsubpart, x1_3 + 180, y1_5 + 4, 50, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(" > ", 50, tBackbitmap: (ref this.OwnBitmap), bbx: (x1_3 + 180), bby: (y1_5 + 4), tinactive: true, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.SourceNextId = this.AddSubPart(ref tsubpart, x1_3 + 180, y1_5 + 4, 50, 35, 0);
      }
      tstring24: String = this.SourcePage.ToString() + "/" + this.SourcePageMax.ToString();
      DrawMod.DrawTextColouredMarcCenter(ref toG, tstring24, this.game.MarcFont4, x1_3 + 120, y1_5 + 12, Color.White);
      bool flag = false;
      SimpleStringList simpleStringList1 = SimpleStringList::new();
      let mut num41: i32 =  0;
      let mut num42: i32 =  0;
      if (this.targetUnr < 0)
        num41 = 1;
      if (this.targetUnr > -1 && this.game.HandyFunctionsObj.CheckIsBattlegroup(this.targetUnr))
        num41 = 1;
      if (this.targetUnr > -1 && this.game.Data.UnitObj[this.targetUnr].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.targetUnr].Historical].Type == 8)
      {
        num41 = 1;
        num42 = 1;
      }
      if (num41 == 0)
      {
        let mut counter7: i32 =  simpleList5.Counter;
        for (let mut index22: i32 =  0; index22 <= counter7; index22 += 1)
        {
          if (simpleList5.Weight[index22] < 0)
          {
            simpleStringList1.Add("Exceeding Target Unit organisational limits.", 1);
            flag = true;
            break;
          }
        }
      }
      if (this.targetUnr != -3 & this.sourceUnr > -1 && num36 < (int) Math.Round( this.game.Data.RuleVar[476]) & !(num36 == 0 & this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.sourceUnr) > 0 & !this.game.Data.UnitObj[this.sourceUnr].IsHQ))
      {
        flag = true;
        SimpleStringList simpleStringList2 = simpleStringList1;
        num25 = (int) Math.Round( this.game.Data.RuleVar[476]);
        tid: String = "You cannot reduce Source Unit below " + num25.ToString() + " power points.";
        simpleStringList2.Add(tid, 1);
      }
      if (this.targetUnr == -2 & num17 < (int) Math.Round( this.game.Data.RuleVar[476]))
      {
        flag = true;
        SimpleStringList simpleStringList3 = simpleStringList1;
        num25 = (int) Math.Round( this.game.Data.RuleVar[476]);
        tid: String = "You cannot create a new Battlegroup with less than " + num25.ToString() + " power points.";
        simpleStringList3.Add(tid, 1);
      }
      if (this.targetUnr == -2 & num17 > (int) Math.Round( this.game.Data.RuleVar[477]))
      {
        flag = true;
        SimpleStringList simpleStringList4 = simpleStringList1;
        num25 = (int) Math.Round( this.game.Data.RuleVar[477]);
        tid: String = "You cannot create a Battlegroup with more than " + num25.ToString() + " power points.";
        simpleStringList4.Add(tid, 1);
      }
      if (this.targetUnr == -1)
      {
        flag = true;
        simpleStringList1.Add("No Target Unit selected.", 1);
      }
      if (battlegroupTemplate > -1 && this.targetUnr == -2 & this.game.Data.RegimeObj[this.game.Data.Turn].ResPts < this.game.Data.HistoricalUnitObj[battlegroupTemplate].PP)
      {
        flag = true;
        simpleStringList1.Add("You do not have the " + this.game.Data.HistoricalUnitObj[battlegroupTemplate].PP.ToString() + " PP to create a new Battlegroup.", 1);
      }
      let mut x1_4: i32 =  245;
      let mut y1_6: i32 =  125 + num11 * 42 + 42;
      let mut h4: i32 =  62;
      let mut w2: i32 =  495;
      DrawMod.DrawBlock(ref toG, x1_4, y1_6, w2, h4, 0, 0, 0, 128);
      let mut num43: i32 =  simpleStringList1.Counter + 1;
      if (num43 < 1)
        num43 = 1;
      if (num43 > 3)
        num43 = 3;
      let mut y: i32 =  y1_6 + (30 - num43 * 10);
      let mut num44: i32 =  x1_4 + 110;
      let mut counter8: i32 =  simpleStringList1.Counter;
      for (let mut index23: i32 =  0; index23 <= counter8; index23 += 1)
      {
        tstring25: String = simpleStringList1.Id[index23];
        DrawMod.DrawTextColouredMarcCenter(ref toG, tstring25, this.game.MarcFont4, num44 + 130, y, Color.Red);
        y += 18;
      }
      if (simpleStringList1.Counter == -1)
      {
        if (this.targetUnr == -3)
        {
          if (this.TL.Counter == -1)
          {
            tstring26: String = "Please select troops for Scrapping.";
            DrawMod.DrawTextColouredMarcCenter(ref toG, tstring26, this.game.MarcFont4, num44 + 130, y, Color.White);
          }
          else
          {
            tstring27: String = "Transfer selection for Scrapping.";
            DrawMod.DrawTextColouredMarcCenter(ref toG, tstring27, this.game.MarcFont4, num44 + 130, y, Color.White);
          }
        }
        else if (this.TL.Counter == -1)
        {
          tstring28: String = "Please select troops to transfer.";
          DrawMod.DrawTextColouredMarcCenter(ref toG, tstring28, this.game.MarcFont4, num44 + 130, y, Color.White);
        }
        else
        {
          tstring29: String = "Transfer selection is valid.";
          DrawMod.DrawTextColouredMarcCenter(ref toG, tstring29, this.game.MarcFont4, num44 + 130, y, Color.White);
        }
      }
      let mut x1_5: i32 =  245;
      let mut y1_7: i32 =  125 + num11 * 42 + 42 + 64;
      let mut h5: i32 =  40;
      DrawMod.DrawBlock(ref toG, x1_5, y1_7, w2, h5, 0, 0, 0, 128);
      let mut num45: i32 =  y1_7 + 4;
      let mut num46: i32 =  x1_5 + 45;
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Cancel", 150, "Click to return to main screen.", ref this.OwnBitmap, num46, num45, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
      this.cancelid = this.AddSubPart(ref tsubpart1, num46, num45, 150, 35, 1);
      let mut num47: i32 =  num46 + 250;
      str1: String = "";
      if (this.targetUnr == -2 & battlegroupTemplate > -1)
        str1 = str1 + " [" + this.game.Data.HistoricalUnitObj[battlegroupTemplate].PP.ToString() + " PP]";
      else if (this.targetUnr <= -1)
        ;
      if (this.targetUnr == -3)
      {
        if (this.TL.Counter > -1 & !flag)
        {
          let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Scrap" + str1, 150, "Go ahead and Scrap the selected equipment.", ref this.OwnBitmap, num47, num45, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          this.transferid = this.AddSubPart(ref tsubpart2, num47, num45, 150, 35, 1);
        }
        else
        {
          let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Scrap" + str1, 150, "Not possible to Scrap.", ref this.OwnBitmap, num47, num45, true, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          this.transfer2id = this.AddSubPart(ref tsubpart3, num47, num45, 150, 35, 0);
        }
      }
      else if (this.TL.Counter > -1 & !flag)
      {
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Transfer" + str1, 150, "Go ahead and make the selected transfer.", ref this.OwnBitmap, num47, num45, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.transferid = this.AddSubPart(ref tsubpart4, num47, num45, 150, 35, 1);
      }
      else
      {
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Transfer" + str1, 150, "Not possible to transfer.", ref this.OwnBitmap, num47, num45, true, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.transfer2id = this.AddSubPart(ref tsubpart5, num47, num45, 150, 35, 0);
      }
      let mut num48: i32 =  5;
      num1 = 980;
      num2 = 110;
      let mut num49: i32 =  10;
      str2: String = "No source unit selected";
      tstring30: String = "";
      tstring31: String = "";
      if (this.sourceUnr > -1)
        str2 = this.game.Data.UnitObj[this.sourceUnr].Name;
      if ( toG.MeasureString(str2, DrawMod.TGame.MarcFont3).Width > 230.0)
      {
        let mut num50: i32 =  Strings.InStr(str2, " ");
        if (num50 > 0)
        {
          let mut num51: i32 =  Strings.InStr(num50 + 1, str2, " ");
          if (num51 > 0)
          {
            let mut num52: i32 =  Strings.InStr(num51 + 1, str2, " ");
            if (num52 > 0)
            {
              tstring31 = Strings.Mid(str2, num52 + 1);
              tstring30 = Strings.Left(str2, num52 - 1);
            }
          }
          else
          {
            let mut num53: i32 =  Strings.InStr(str2, " ");
            if (num53 <= 0 || Strings.InStr(num53 + 1, str2, " ") <= 0)
              ;
          }
        }
        if (tstring30.Length > 0)
        {
          DrawMod.DrawTextColouredMarcCenter(ref toG, tstring30, this.game.MarcFont4, num48 + 360, num49 + 20, Color.White);
          DrawMod.DrawTextColouredMarcCenter(ref toG, tstring31, this.game.MarcFont4, num48 + 360, num49 + 35, Color.White);
        }
        else
          DrawMod.DrawTextColouredMarcCenter(ref toG, str2, this.game.MarcFont4, num48 + 360, num49 + 30, Color.White);
      }
      else
        DrawMod.DrawTextColouredMarcCenter(ref toG, str2, this.game.MarcFont3, num48 + 360, num49 + 30, Color.White);
      tstring32: String = "Wgt: " + num33.ToString() + " Car: " + num34.ToString() + " Pow: " + num36.ToString();
      DrawMod.DrawTextColouredMarcCenter(ref toG, tstring32, this.game.MarcFont4, num48 + 360, num49 + 60, Color.White);
      if (num32 > 0 | num35 > 0)
      {
        tstring33: String = "ManpWgt: " + num32.ToString() + " ManpCar: " + num35.ToString();
        DrawMod.DrawTextColouredMarcCenter(ref toG, tstring33, this.game.MarcFont4, num48 + 360, num49 + 80, Color.White);
      }
      str3: String = "No target unit selected";
      if (this.targetUnr > -1)
        str3 = this.game.Data.UnitObj[this.targetUnr].Name;
      if (this.targetUnr == -2)
        str3 = "New Battlegroup";
      if (this.targetUnr == -3)
        str3 = "Selected for Scrapping";
      if (this.targetUnr == -2 && battlegroupTemplate > -1)
        str3 = "New " + this.game.Data.HistoricalUnitObj[battlegroupTemplate].Name;
      tstring34: String = "";
      tstring35: String = "";
      if ( toG.MeasureString(str3, DrawMod.TGame.MarcFont3).Width > 230.0)
      {
        let mut num54: i32 =  Strings.InStr(str3, " ");
        if (num54 > 0)
        {
          let mut num55: i32 =  Strings.InStr(num54 + 1, str3, " ");
          if (num55 > 0)
          {
            let mut num56: i32 =  Strings.InStr(num55 + 1, str3, " ");
            if (num56 > 0)
            {
              tstring35 = Strings.Mid(str3, num56 + 1);
              tstring34 = Strings.Left(str3, num56 - 1);
            }
          }
          else
          {
            let mut num57: i32 =  Strings.InStr(str3, " ");
            if (num57 <= 0 || Strings.InStr(num57 + 1, str3, " ") <= 0)
              ;
          }
        }
        if (tstring34.Length > 0)
        {
          DrawMod.DrawTextColouredMarcCenter(ref toG, tstring34, this.game.MarcFont4, num48 + 600, num49 + 20, Color.White);
          DrawMod.DrawTextColouredMarcCenter(ref toG, tstring35, this.game.MarcFont4, num48 + 600, num49 + 35, Color.White);
        }
        else
          DrawMod.DrawTextColouredMarcCenter(ref toG, str3, this.game.MarcFont4, num48 + 600, num49 + 30, Color.White);
      }
      else
        DrawMod.DrawTextColouredMarcCenter(ref toG, str3, this.game.MarcFont3, num48 + 600, num49 + 30, Color.White);
      tstring36: String = "Wgt: " + num13.ToString() + " Car: " + num15.ToString() + " Pow: " + num17.ToString();
      DrawMod.DrawTextColouredMarcCenter(ref toG, tstring36, this.game.MarcFont4, num48 + 600, num49 + 60, Color.White);
      if (!(num14 > 0 | num16 > 0))
        return;
      tstring37: String = "ManpWgt: " + num14.ToString() + " ManpCar: " + num16.ToString();
      DrawMod.DrawTextColouredMarcCenter(ref toG, tstring37, this.game.MarcFont4, num48 + 600, num49 + 80, Color.White);
    }

    pub HandleKeyPress: WindowReturnClass(int nr, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseUp: WindowReturnClass(int x, int y, int b)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult = OrderResult::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
label_10:
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (this.SubPartList[index1].Scroller)
          {
            let mut index2: i32 =  0;
            while (this.SlidersId[index2] != this.SubPartID[index1])
            {
              index2 += 1;
              if (index2 > 19)
                goto label_10;
            }
            let mut tweight: i32 =  (int) Math.Round( this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1], b) /  this.game.Data.SFTypeObj[this.game.Data.SFObj[this.SlidersSfnr[index2]].Type].Ratio);
            this.SubPartFlag[index1] = true;
            this.FlagAll();
            windowReturnClass.SetFlag(true);
            this.SubPartList[index1].Scroller = false;
            let mut nr: i32 =  this.TL.FindNr(this.SlidersSfnr[index2]);
            if (nr > -1)
              this.TL.RemoveNr(nr);
            this.TL.Add(this.SlidersSfnr[index2], tweight);
            this.View();
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(int x, int y, int b)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (this.MouseData[index] > 0 && x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] == 1)
          {
            this.SetUnits();
            this.TL = SimpleList::new();
            this.sourceUnr = this.MouseData2[index];
            this.View();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (this.MouseData[index] == 2)
          {
            this.SetUnits();
            this.TL = SimpleList::new();
            this.targetUnr = this.MouseData2[index];
            this.View();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
label_125:
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut index2: i32 =  0;
            while (this.SubPartID[index1] != this.SlidersId[index2])
            {
              index2 += 1;
              if (index2 > 19)
              {
                let mut num1: i32 =  this.SubPartID[index1];
                if (num1 == this.SourcePrevId)
                {
                  this.SetUnits();
                  --this.SourcePage;
                  this.FlagAll();
                  windowReturnClass.SetFlag(true);
                  this.View();
                  return windowReturnClass;
                }
                if (num1 == this.SourceNextId)
                {
                  this.SetUnits();
                  this += 1.SourcePage;
                  this.FlagAll();
                  windowReturnClass.SetFlag(true);
                  this.View();
                  return windowReturnClass;
                }
                if (num1 == this.TargetPrevId)
                {
                  --this.TargetPage;
                  this.FlagAll();
                  windowReturnClass.SetFlag(true);
                  this.View();
                  return windowReturnClass;
                }
                if (num1 == this.TargetNextId)
                {
                  this += 1.TargetPage;
                  this.FlagAll();
                  windowReturnClass.SetFlag(true);
                  this.View();
                  return windowReturnClass;
                }
                if (num1 == this.cancelid)
                {
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.transferid)
                {
                  let mut unr: i32 =  -1;
                  let mut battlegroupTemplate: i32 =  this.game.HandyFunctionsObj.GetBattlegroupTemplate(this.game.Data.Turn);
                  let mut sfCount1: i32 =  this.game.Data.UnitObj[this.sourceUnr].SFCount;
                  int num2;
                  int num3;
                  for (let mut index3: i32 =  0; index3 <= sfCount1; index3 += 1)
                  {
                    let mut sf: i32 =  this.game.Data.UnitObj[this.sourceUnr].SFList[index3];
                    let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
                    let mut type: i32 =  this.game.Data.SFObj[sf].Type;
                    num2 += this.game.Data.SFTypeObj[type].FuelCarry * qty;
                    num3 += this.game.Data.SFTypeObj[type].SupplyCarry * qty;
                  }
                  if (this.targetUnr == -3)
                  {
                    let mut counter1: i32 =  this.TL.Counter;
                    for (let mut index4: i32 =  0; index4 <= counter1; index4 += 1)
                    {
                      this.TL.Data5[index4] = this.TL.Weight[index4];
                      this.TL.Weight[index4] = this.TL.Id[index4];
                    }
                    this.TL.ReverseSort();
                    let mut counter2: i32 =  this.TL.Counter;
                    for (let mut index5: i32 =  0; index5 <= counter2; index5 += 1)
                      this.TL.Weight[index5] = this.TL.Data5[index5];
                    if ( this.game.Data.RuleVar[486] > 0.0)
                    {
                      let mut stringListById: i32 =  this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[486]));
                      let mut counter3: i32 =  this.TL.Counter;
                      for (let mut index6: i32 =  0; index6 <= counter3; index6 += 1)
                      {
                        let mut index7: i32 =  this.TL.Id[index6];
                        let mut qty: i32 =  this.game.Data.SFObj[index7].Qty;
                        let mut people: i32 =  this.game.Data.SFObj[index7].People;
                        let mut type: i32 =  this.game.Data.SFObj[index7].Type;
                        let mut xp: i32 =  this.game.Data.SFObj[index7].Xp;
                        let mut ap: i32 =  this.game.Data.SFObj[index7].Ap;
                        let mut rdn: i32 =  this.game.Data.SFObj[index7].Rdn;
                        let mut mor: i32 =  this.game.Data.SFObj[index7].Mor;
                        let mut supplyConsume: i32 =  this.game.Data.UnitObj[this.sourceUnr].SupplyConsume;
                        let mut offMod: i32 =  this.game.Data.SFObj[index7].OffMod;
                        let mut defMod: i32 =  this.game.Data.SFObj[index7].DefMod;
                        let mut SfType: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].GetData(0, people, 1)));
                        let mut Qty: i32 =  0;
                        if (this.game.Data.SFTypeObj[type].scrapable >= 100)
                          Qty = this.TL.Weight[index6] * (int) Math.Round(Math.Floor( this.game.Data.SFTypeObj[type].scrapable / 100.0));
                        else if (this.game.Data.SFTypeObj[type].scrapable > 1)
                        {
                          let mut num4: i32 =  this.TL.Weight[index6];
                          for (let mut index8: i32 =  1; index8 <= num4; index8 += 1)
                          {
                            if (DrawMod.RandyNumber.Next(1, 100) <= this.game.Data.SFTypeObj[type].scrapable)
                              Qty += 1;
                          }
                        }
                        else
                          Qty = 0;
                        let mut Ap: i32 =  ap - 25;
                        if (Ap < 0)
                          Ap = 0;
                        this.game.HandyFunctionsObj.AddTroops3(this.sourceUnr, SfType, people, Qty, xp, rdn, Ap, mor, supplyConsume, offmod: offMod, defmod: defMod);
                        this.game.HandyFunctionsObj.RemoveTroops(this.sourceUnr, type, people, this.TL.Weight[index6], -1);
                      }
                      let mut num5: i32 =  0;
                      let mut num6: i32 =  0;
                      let mut sfCount2: i32 =  this.game.Data.UnitObj[this.sourceUnr].SFCount;
                      for (let mut index9: i32 =  0; index9 <= sfCount2; index9 += 1)
                      {
                        let mut sf: i32 =  this.game.Data.UnitObj[this.sourceUnr].SFList[index9];
                        let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
                        let mut type: i32 =  this.game.Data.SFObj[sf].Type;
                        num5 += this.game.Data.SFTypeObj[type].SupplyCarry * qty;
                        num6 += this.game.Data.SFTypeObj[type].FuelCarry * qty;
                      }
                      if (num5 < this.game.Data.UnitObj[this.sourceUnr].Supply)
                        this.game.Data.UnitObj[this.sourceUnr].Supply = num5;
                      if (num6 < this.game.Data.UnitObj[this.sourceUnr].Fuel)
                        this.game.Data.UnitObj[this.sourceUnr].Fuel = num6;
                    }
                  }
                  else
                  {
                    if (this.targetUnr == -2)
                    {
                      this.game.ProcessingObj.AddNewUnitBasedOnHistorical(this.game.SelectX, this.game.SelectY, 0, this.game.Data.Turn, battlegroupTemplate);
                      unr = this.game.Data.UnitCounter;
                      let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
                      if ( this.game.Data.RuleVar[403] == 1.0)
                      {
                        this.game.Data.HistoricalUnitObj[historicalUnitCounter].SetHisVarValue(71, 50);
                        this.game.Data.HistoricalUnitObj[historicalUnitCounter].SetHisVarValue(72, 10);
                      }
                      this.game.Data.UnitObj[unr].HQ = this.game.Data.UnitObj[this.sourceUnr].HQ;
                      if (this.game.Data.Product == 7 && this.game.Data.UnitObj[this.sourceUnr].HQ > -1 && this.game.Data.UnitObj[this.game.Data.UnitObj[this.sourceUnr].HQ].HQ > -1)
                        this.game.Data.UnitObj[unr].HQ = this.game.Data.UnitObj[this.game.Data.UnitObj[this.sourceUnr].HQ].HQ;
                      this.game.Data.HistoricalUnitObj[historicalUnitCounter].BattleGroup = 1;
                      if ( this.game.Data.RuleVar[479] > 0.0)
                      {
                        let mut stringListById: i32 =  this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[479]));
                        str: String;
                        int num7;
                        for (let mut index10: i32 =  0; index10 < 99; index10 += 1)
                        {
                          let mut index11: i32 =  DrawMod.RandyNumber.Next(0, this.game.Data.StringListObj[stringListById].Length);
                          if (Conversions.ToInteger(this.game.Data.StringListObj[stringListById].Data[index11, 0]) == this.game.Data.Turn)
                          {
                            str = this.game.Data.StringListObj[stringListById].Data[index11, 1];
                            num7 = 0;
                            let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                            for (let mut index12: i32 =  0; index12 <= unitCounter; index12 += 1)
                            {
                              if (Strings.InStr(this.game.Data.UnitObj[index12].Name, str) > 0)
                                num7 = 1;
                            }
                            if (num7 == 0)
                              index10 = 99999;
                          }
                        }
                        if (num7 == 1)
                        {
                          this.game.Data.HistoricalUnitObj[historicalUnitCounter].CounterString = Strings.Left(this.game.Data.HistoricalUnitObj[battlegroupTemplate].Name, 5);
                        }
                        else
                        {
                          this.game.Data.UnitObj[unr].Name = this.game.Data.HistoricalUnitObj[battlegroupTemplate].Name + " " + str;
                          this.game.Data.HistoricalUnitObj[historicalUnitCounter].Name = this.game.Data.UnitObj[unr].Name;
                          this.game.Data.HistoricalUnitObj[historicalUnitCounter].CounterString = Strings.Left(str, 5);
                        }
                      }
                    }
                    else
                      unr = this.targetUnr;
                    SimpleList SL1 = SimpleList::new();
                    SimpleList simpleList1;
                    SimpleList simpleList2;
                    if ( this.game.Data.RuleVar[403] == 1.0)
                    {
                      let mut sfCount3: i32 =  this.game.Data.UnitObj[this.sourceUnr].SFCount;
                      for (let mut index13: i32 =  0; index13 <= sfCount3; index13 += 1)
                      {
                        let mut sf: i32 =  this.game.Data.UnitObj[this.sourceUnr].SFList[index13];
                        let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
                        let mut people: i32 =  this.game.Data.SFObj[sf].People;
                        let mut type: i32 =  this.game.Data.SFObj[sf].Type;
                        SL1.AddWeight(type, qty);
                      }
                      simpleList1 = DrawMod.TGame.EventRelatedObj.Helper_SupplyItemList_ForSFTypeList(SL1);
                      simpleList2 = this.game.Data.UnitObj[this.sourceUnr].items.list.Clone();
                    }
                    let mut counter: i32 =  this.TL.Counter;
                    int num8;
                    int num9;
                    for (let mut index14: i32 =  0; index14 <= counter; index14 += 1)
                    {
                      let mut index15: i32 =  this.TL.Id[index14];
                      let mut qty: i32 =  this.game.Data.SFObj[index15].Qty;
                      let mut people: i32 =  this.game.Data.SFObj[index15].People;
                      let mut type: i32 =  this.game.Data.SFObj[index15].Type;
                      let mut xp: i32 =  this.game.Data.SFObj[index15].Xp;
                      let mut rdn: i32 =  this.game.Data.SFObj[index15].Rdn;
                      let mut mor: i32 =  this.game.Data.SFObj[index15].Mor;
                      let mut supplyConsume: i32 =  this.game.Data.UnitObj[this.sourceUnr].SupplyConsume;
                      let mut offMod: i32 =  this.game.Data.SFObj[index15].OffMod;
                      let mut defMod: i32 =  this.game.Data.SFObj[index15].DefMod;
                      num8 += this.game.Data.SFTypeObj[type].FuelCarry * this.TL.Weight[index14];
                      num9 += this.game.Data.SFTypeObj[type].SupplyCarry * this.TL.Weight[index14];
                      if ( this.game.Data.RuleVar[403] == 1.0)
                      {
                        SimpleList SL2 = SimpleList::new();
                        let mut tid1: i32 =  this.game.Data.SFTypeObj[type].SFTypeVar[45];
                        let mut tweight1: i32 =  this.game.Data.SFTypeObj[type].SFTypeVar[52] * this.TL.Weight[index14];
                        if (tid1 > 0 & simpleList1.FindWeight(tid1) > 0)
                          tweight1 = (int) Math.Round(Math.Floor( tweight1 * Math.Min(1.0,  simpleList2.FindWeight(tid1) /  simpleList1.FindWeight(tid1))));
                        if (tweight1 > 0)
                          SL2.Add(tid1, tweight1);
                        let mut tid2: i32 =  this.game.Data.SFTypeObj[type].SFTypeVar[47];
                        let mut tweight2: i32 =  this.game.Data.SFTypeObj[type].SFTypeVar[53] * this.TL.Weight[index14];
                        if (tid2 > 0 & simpleList1.FindWeight(tid2) > 0)
                          tweight2 = (int) Math.Round(Math.Floor( tweight2 * Math.Min(1.0,  simpleList2.FindWeight(tid2) /  simpleList1.FindWeight(tid2))));
                        if (tweight2 > 0)
                          SL2.Add(tid2, tweight2);
                        let mut tid3: i32 =  this.game.Data.SFTypeObj[type].SFTypeVar[50];
                        let mut tweight3: i32 =  this.game.Data.SFTypeObj[type].SFTypeVar[54] * this.TL.Weight[index14];
                        if (tid3 > 0 & simpleList1.FindWeight(tid3) > 0)
                          tweight3 = (int) Math.Round(Math.Floor( tweight3 * Math.Min(1.0,  simpleList2.FindWeight(tid3) /  simpleList1.FindWeight(tid3))));
                        if (tweight3 > 0)
                          SL2.Add(tid3, tweight3);
                        if (this.game.Data.UnitObj[this.sourceUnr].items.list.CanRemoveWeight(ref SL2))
                        {
                          this.game.Data.UnitObj[this.sourceUnr].items.list.RemoveWeight(ref SL2);
                          this.game.Data.UnitObj[unr].items.list.AddWeight(ref SL2);
                        }
                      }
                      this.game.HandyFunctionsObj.AddTroops3(unr, type, people, this.TL.Weight[index14], xp, rdn, 0, mor, supplyConsume, offmod: offMod, defmod: defMod);
                      this.game.HandyFunctionsObj.RemoveTroops(this.sourceUnr, type, people, this.TL.Weight[index14], -1);
                    }
                    if ( this.game.Data.RuleVar[403] == 0.0)
                    {
                      if (num3 > 0)
                      {
                        let mut num10: i32 =  (int) Math.Round( (this.game.Data.UnitObj[this.sourceUnr].Supply * num9) /  num3);
                        UnitClass[] unitObj1 = this.game.Data.UnitObj;
                        UnitClass[] unitClassArray1 = unitObj1;
                        let mut index16: i32 =  unr;
                        let mut index17: i32 =  index16;
                        unitClassArray1[index17].Supply = unitObj1[index16].Supply + num10;
                        UnitClass[] unitObj2 = this.game.Data.UnitObj;
                        UnitClass[] unitClassArray2 = unitObj2;
                        let mut sourceUnr: i32 =  this.sourceUnr;
                        let mut index18: i32 =  sourceUnr;
                        unitClassArray2[index18].Supply = unitObj2[sourceUnr].Supply - num10;
                        if (this.game.Data.UnitObj[this.sourceUnr].Supply < 0)
                          this.game.Data.UnitObj[this.sourceUnr].Supply = 0;
                      }
                      if (num2 > 0)
                      {
                        let mut num11: i32 =  (int) Math.Round( (this.game.Data.UnitObj[this.sourceUnr].Fuel * num8) /  num2);
                        UnitClass[] unitObj3 = this.game.Data.UnitObj;
                        UnitClass[] unitClassArray3 = unitObj3;
                        let mut index19: i32 =  unr;
                        let mut index20: i32 =  index19;
                        unitClassArray3[index20].Fuel = unitObj3[index19].Fuel + num11;
                        UnitClass[] unitObj4 = this.game.Data.UnitObj;
                        UnitClass[] unitClassArray4 = unitObj4;
                        let mut sourceUnr: i32 =  this.sourceUnr;
                        let mut index21: i32 =  sourceUnr;
                        unitClassArray4[index21].Fuel = unitObj4[sourceUnr].Fuel - num11;
                        if (this.game.Data.UnitObj[this.sourceUnr].Fuel < 0)
                          this.game.Data.UnitObj[this.sourceUnr].Fuel = 0;
                      }
                    }
                    if (this.game.Data.UnitObj[this.sourceUnr].SFCount == -1 & !this.game.Data.UnitObj[this.sourceUnr].IsHQ)
                    {
                      this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].RemoveUnitFromList(this.sourceUnr);
                      if (this.game.EditObj.UnitSelected == this.sourceUnr)
                        this.game.EditObj.UnitSelected = -1;
                      if (this.game.EditObj.OrderUnit == this.sourceUnr)
                        this.game.EditObj.OrderUnit = -1;
                      this.game.EditObj.UnitSelected = -1;
                      this.game.EditObj.OrderUnit = -1;
                      data: DataClass = this.game.Data;
                      let mut sourceUnr: i32 =  this.sourceUnr;
                      let mut gameClass: GameClass = (GameClass) null;
                      ref let mut local: GameClass = ref gameClass;
                      data.RemoveUnit(sourceUnr, ref local);
                      this.game.EditObj.OldUnit = -1;
                      this.sourceUnr = -1;
                      this.targetUnr = -1;
                      unr = -1;
                    }
                  }
                  if (this.sourceUnr > -1 && !Information.IsNothing( this.game.Data.UnitObj[this.sourceUnr].tempSFTypeBitmap))
                  {
                    this.game.Data.UnitObj[this.sourceUnr].tempSFTypeBitmap.Dispose();
                    this.game.Data.UnitObj[this.sourceUnr].tempSFTypeBitmap = (Bitmap) null;
                  }
                  if (this.targetUnr > -1 && !Information.IsNothing( this.game.Data.UnitObj[this.targetUnr].tempSFTypeBitmap))
                  {
                    this.game.Data.UnitObj[this.targetUnr].tempSFTypeBitmap.Dispose();
                    this.game.Data.UnitObj[this.targetUnr].tempSFTypeBitmap = (Bitmap) null;
                  }
                  this.SetUnits();
                  if (this.targetUnr != -3)
                    this.targetUnr = unr;
                  this.TL = SimpleList::new();
                  this.FlagAll();
                  windowReturnClass.SetFlag(true);
                  this.View();
                  return windowReturnClass;
                }
                goto label_125;
              }
            }
            let mut num: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1], b);
            let mut nr: i32 =  this.TL.FindNr(this.SlidersSfnr[index2]);
            let mut tweight: i32 =  (int) Math.Round( num /  this.game.Data.SFTypeObj[this.game.Data.SFObj[this.SlidersSfnr[index2]].Type].Ratio);
            if (nr > -1)
              this.TL.RemoveNr(nr);
            this.TL.Add(this.SlidersSfnr[index2], tweight);
            this.SubPartFlag[index1] = true;
            this.FlagAll();
            windowReturnClass.SetFlag(true);
            this.SubPartList[index1].Scroller = true;
            this.View();
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
