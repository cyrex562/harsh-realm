// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CombatSelectWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;

namespace WindowsApplication1
{
  pub class CombatSelectWindowClass : WindowClass
  {
     okid: i32;
     cancelid: i32;
     mapButId: i32;
     oktextid: i32;
     UnitList UL;
     UnitList DL;
     oldUnitSelected: i32;
     oldSelectx: i32;
     oldSelecty: i32;
     oldMode1: i32;
     oldMode2: i32;
     CoordList oldTempCoordList;
     CoordList oldTempCoordLastMoveList;
     ListClass rlistobj;
     ListClass rlist2obj;
     rlistid: i32;
     rlist2id: i32;
     switchId: i32;
     UnitList EL;

    pub CombatSelectWindowClass(ref tGame: GameClass)
      : base(ref tGame, 1280, 768, 8)
    {
      this.oldMode1 = this.game.EditObj.HideUnit;
      this.oldMode2 = -(this.game.EditObj.SpreadUnit ? 1 : 0);
      this.game.EditObj.HideUnit = 1;
      this.game.EditObj.SpreadUnit = true;
      this.oldUnitSelected = this.game.EditObj.UnitSelected;
      this.oldSelectx = this.game.SelectX;
      this.oldSelecty = this.game.SelectY;
      this.oldTempCoordList = this.game.EditObj.TempCoordList;
      this.oldTempCoordLastMoveList = this.game.EditObj.TempCoordListLastMove;
      if (Information.IsNothing( this.game.EditObj.TempUnitList))
        this.game.EditObj.TempUnitList = UnitList::new();
      this.Setup();
      this.View();
    }

    pub fn Setup()
    {
      this.EL = UnitList::new();
      let mut unitCounter1: i32 =  this.game.Data.UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter1; index += 1)
      {
        this.game.Data.UnitObj[index].tempCanAttack = false;
        this.game.Data.UnitObj[index].tempCanAttack2 = false;
      }
      Coordinate target;
      target.x = this.game.SelectX;
      target.y = this.game.SelectY;
      if (this.game.EditObj.OrderType == 2)
      {
        let mut tfacing: i32 =  1;
        do
        {
          Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, 0, tfacing);
          if (coordinate.onmap)
          {
            let mut unitCounter2: i32 =  this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitCounter;
            for (let mut index: i32 =  0; index <= unitCounter2; index += 1)
            {
              let mut unit: i32 =  this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitList[index];
              if ( this.game.Data.RuleVar[307] <=  this.game.HandyFunctionsObj.GetPowerPtsAbsolute(unit) && this.game.HandyFunctionsObj.CanDoLandAttack(unit, target))
              {
                this.EL.add(unit);
                this.game.Data.UnitObj[unit].tempCanAttack = true;
              }
            }
          }
          tfacing += 1;
        }
        while (tfacing <= 6);
      }
      if (this.game.EditObj.OrderType == 11)
      {
        let mut x: i32 =  target.x;
        let mut y: i32 =  target.y;
        let mut mapWidth: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          {
            let mut x2: i32 =  index1;
            let mut y2: i32 =  index2;
            if (this.game.Data.MapObj[0].HexObj[x2, y2].Regime == this.game.Data.Turn & this.game.Data.MapObj[0].HexObj[x2, y2].UnitCounter > -1)
            {
              let mut unitCounter3: i32 =  this.game.Data.MapObj[0].HexObj[x2, y2].UnitCounter;
              for (let mut index3: i32 =  0; index3 <= unitCounter3; index3 += 1)
              {
                let mut unit: i32 =  this.game.Data.MapObj[0].HexObj[x2, y2].UnitList[index3];
                if (this.game.Data.UnitObj[unit].Regime == this.game.Data.Turn)
                {
                  let mut maxArtRange: i32 =  this.game.HandyFunctionsObj.GetMaxArtRange(unit, 0);
                  if (maxArtRange > 0 && this.game.HandyFunctionsObj.Distance(x, y, 0, x2, y2, 0, 899) <= maxArtRange && this.game.HandyFunctionsObj.CanDoArtAttack(unit, target, false))
                  {
                    this.EL.add(unit);
                    this.game.Data.UnitObj[unit].tempCanAttack = true;
                  }
                }
              }
            }
          }
        }
      }
      if (this.game.EditObj.OrderType == 14 | this.game.EditObj.OrderType == 33)
      {
        let mut x: i32 =  target.x;
        let mut y: i32 =  target.y;
        let mut mapWidth: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index4: i32 =  0; index4 <= mapWidth; index4 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
          {
            let mut x2: i32 =  index4;
            let mut y2: i32 =  index5;
            if (this.game.Data.MapObj[0].HexObj[x2, y2].Regime == this.game.Data.Turn & this.game.Data.MapObj[0].HexObj[x2, y2].UnitCounter > -1)
            {
              let mut unitCounter4: i32 =  this.game.Data.MapObj[0].HexObj[x2, y2].UnitCounter;
              for (let mut index6: i32 =  0; index6 <= unitCounter4; index6 += 1)
              {
                let mut unit: i32 =  this.game.Data.MapObj[0].HexObj[x2, y2].UnitList[index6];
                if (this.game.Data.UnitObj[unit].Regime == this.game.Data.Turn)
                {
                  let mut maxAirRange: i32 =  this.game.HandyFunctionsObj.GetMaxAirRange(unit);
                  if (maxAirRange > 0 && this.game.HandyFunctionsObj.Distance(x, y, 0, x2, y2, 0, 899) <= maxAirRange && this.game.HandyFunctionsObj.CanDoAirStrike(unit, target))
                  {
                    bool flag = false;
                    let mut minimumAirfieldLevel: i32 =  this.game.HandyFunctionsObj.SE1_GetUnitMinimumAirfieldLevel(unit);
                    if (minimumAirfieldLevel > 0)
                    {
                      if (this.game.Data.MapObj[0].HexObj[x2, y2].Location > -1)
                      {
                        if (this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[x2, y2].Location].tempAirfieldLevel < minimumAirfieldLevel)
                          flag = true;
                      }
                      else
                        flag = true;
                    }
                    if (!flag)
                    {
                      this.EL.add(unit);
                      this.game.Data.UnitObj[unit].tempCanAttack = true;
                    }
                  }
                }
              }
            }
          }
        }
      }
      for (let mut counter: i32 =  this.game.EditObj.TempUnitList.counter; counter >= 0; counter += -1)
      {
        let mut tunr: i32 =  this.game.EditObj.TempUnitList.unr[counter];
        if (!this.EL.CheckIfPresent(tunr) | this.game.Data.UnitObj[tunr].tempCanAttack2)
          this.game.EditObj.TempUnitList.remove(tunr);
      }
    }

    pub fn HandleToolTip(x: i32, y: i32)
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
      if (this.cancelid > 0)
      {
        this.RemoveSubPart(this.cancelid);
        this.cancelid = 0;
      }
      if (this.okid > 0)
      {
        this.RemoveSubPart(this.okid);
        this.okid = 0;
      }
      if (this.switchId > 0)
      {
        this.RemoveSubPart(this.switchId);
        this.switchId = 0;
      }
      if (this.mapButId > 0)
      {
        this.RemoveSubPart(this.mapButId);
        this.mapButId = 0;
      }
      this.ClearMouse();
      this.NewBackGroundAndClearAll(1280, 768, 8);
      Graphics toG = Graphics.FromImage((Image) this.OwnBitmap);
      toG.Clear(Color.Transparent);
      DrawMod.DrawMessFrame(ref this.OwnBitmap, ref toG, 70, 60, 1140, 678);
      DrawMod.DrawBlock(ref toG, 79, 66, 1115, 212, 0, 0, 0, 128);
      DrawMod.DrawBlock(ref toG, 79, 66, 1115, 188, 0, 0, 0, 128);
      DrawMod.DrawBlock(ref toG, 79, 499, 1115, 234, 0, 0, 0, 128);
      DrawMod.DrawBlock(ref toG, 443, 220, 410, 320, 0, 0, 0,  byte.MaxValue);
      ref Graphics local1 = ref toG;
      bitmap1: Bitmap = BitmapStore.GetBitmap(this.game.SE1_COMPLEXFRAME);
      ref local2: Bitmap = ref bitmap1;
      DrawMod.DrawSimple(ref local1, ref local2, 408, 205);
      SimpleStringList simpleStringList = this.game.HandyFunctionsObj.CombatPreviewStats(this.game.SelectX, this.game.SelectY);
      bool flag1 = false;
      let mut num1: i32 =  Math.Max(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter + 1, this.EL.counter + 1);
      bool flag2 = num1 <= 8;
      flag1 = num1 > 28;
      SizeF sizeF1 = SizeF::new();
      str1: String = "???";
      if (this.game.EditObj.OrderType == 2)
        str1 = "Regular Attack";
      if (this.game.EditObj.OrderType == 11)
        str1 = "Ranged Attack";
      if (this.game.EditObj.OrderType == 14)
        str1 = "Air Attack";
      if (this.game.EditObj.OrderType == 33)
        str1 = "Air Recon";
      SizeF sizeF2 = toG.MeasureString(str1, this.game.MarcFont1);
      DrawMod.DrawBlock(ref toG, 450, 80, 376, 50, 0, 0, 0, 155);
      DrawMod.DrawRectangle(ref toG, 450, 80, 376, 50,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 85, 2);
      DrawMod.DrawTextColouredMarc(ref toG, str1, this.game.MarcFont1,  Math.Round(640.0 -  sizeF2.Width / 2.0), 90, Color.White);
      let mut x1: i32 =  136;
      let mut y1: i32 =  152;
      let mut num2: i32 =  x1;
      let mut num3: i32 =  y1;
      bitmap2: Bitmap = new Bitmap(400, 400, PixelFormat.Format32bppPArgb);
      bitmap2.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics objGraphics = Graphics.FromImage((Image) bitmap2);
      objGraphics.CompositingMode = CompositingMode.SourceCopy;
      objGraphics.Clear(Color.Transparent);
      objGraphics.CompositingMode = CompositingMode.SourceOver;
      objBitmap: Bitmap = new Bitmap(128, 96, PixelFormat.Format32bppPArgb);
      objBitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics = Graphics.FromImage((Image) objBitmap);
      customBitmapObj1: CustomBitmapClass = this.game.CustomBitmapObj;
      let mut selectX: i32 =  this.game.SelectX;
      let mut selectY: i32 =  this.game.SelectY;
      Graphics tempg1 = graphics;
      bitmap3: Bitmap = (Bitmap) null;
      ref local3: Bitmap = ref bitmap3;
      bool flag3 = false;
      ref bool local4 = ref flag3;
      customBitmapObj1.DrawHex(selectX, selectY, 0, tempg: tempg1, counteralpha: 0, Zoom: 1, neverusehistory: true, combatSetup: true, gBitmap: (ref local3), tFromMapPopup: (ref local4));
      DrawMod.DrawSimple(ref objGraphics, ref objBitmap, x1, y1);
      let mut tfacing: i32 =  1;
      bitmap4: Bitmap;
      do
      {
        Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, 0, tfacing);
        if (coordinate.onmap)
        {
          let mut x2: i32 =  num2;
          let mut y2: i32 =  num3;
          if (tfacing == 1)
            y2 -= 96;
          if (tfacing == 2)
          {
            x2 += 106;
            y2 -= 48;
          }
          if (tfacing == 3)
          {
            x2 += 106;
            y2 += 48;
          }
          if (tfacing == 4)
            y2 += 96;
          if (tfacing == 5)
          {
            x2 -= 106;
            y2 += 48;
          }
          if (tfacing == 6)
          {
            x2 -= 106;
            y2 -= 48;
          }
          customBitmapObj2: CustomBitmapClass = this.game.CustomBitmapObj;
          let mut x3: i32 =  coordinate.x;
          let mut y3: i32 =  coordinate.y;
          Graphics tempg2 = graphics;
          bitmap4 = (Bitmap) null;
          ref local5: Bitmap = ref bitmap4;
          flag3 = false;
          ref bool local6 = ref flag3;
          customBitmapObj2.DrawHex(x3, y3, 0, tempg: tempg2, counteralpha: 0, Zoom: 1, neverusehistory: true, combatSetup: true, gBitmap: (ref local5), tFromMapPopup: (ref local6));
          DrawMod.DrawSimple(ref objGraphics, ref objBitmap, x2, y2);
        }
        tfacing += 1;
      }
      while (tfacing <= 6);
      toG.DrawImage((Image) bitmap2, 438, 175);
      bitmap2.Dispose();
      objGraphics.Dispose();
      objBitmap.Dispose();
      graphics.Dispose();
      let mut num4: i32 =  0;
      let mut num5: i32 =  842;
      let mut num6: i32 =  534;
      if (this.game.SelectX == -1)
        return;
      let mut landscapeType: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType;
      let mut spriteNr: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SpriteNr;
      let mut num7: i32 =  -1;
      let mut num8: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter;
      if (num8 > 15)
        num8 = 15;
      let mut num9: i32 =  num8;
      unit1: i32;
      for (let mut index: i32 =  0; index <= num9; index += 1)
      {
        unit1 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitList[index];
        if (this.game.HandyFunctionsObj.CanWeSeeUnit(unit1, this.game.Data.Turn) > 0)
          num7 += 1;
      }
      num10: i32;
      num11: i32;
      Rectangle trect1;
      Rectangle trect2;
      if (landscapeType > -1 & spriteNr > -1 && this.game.Data.Product > 6 &  this.game.Data.RuleVar[410] > 0.0)
      {
        ref Graphics local7 = ref toG;
        bitmap4 = BitmapStore.GetBitmap(this.game.SE1_SUPERIMPOSEBACKGROUND);
        ref local8: Bitmap = ref bitmap4;
        let mut x4: i32 =  num5;
        let mut y4: i32 =  num6;
        DrawMod.DrawSimple(ref local7, ref local8, x4, y4);
        ref Graphics local9 = ref toG;
        bitmap4 = BitmapStore.GetBitmap(this.game.SE1_SIDEBARHEADER);
        ref local10: Bitmap = ref bitmap4;
        let mut x5: i32 =  num5 + 120;
        let mut y5: i32 =  num6 - 20;
        DrawMod.DrawSimple(ref local9, ref local10, x5, y5);
        str2: String = "Enemy Forces";
        SizeF sizeF3 = toG.MeasureString(str2, this.game.MarcFont4);
        DrawMod.DrawTextColouredMarc(ref toG, str2, this.game.MarcFont4,  Math.Round( ( (num5 + 120 + 70) - sizeF3.Width / 2f)), num6 - 10, Color.White);
        let mut num12: i32 =  -1;
        let mut num13: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter;
        if (num13 > 15)
          num13 = 15;
        let mut num14: i32 =  num13;
        for (let mut index: i32 =  0; index <= num14; index += 1)
        {
          unit1 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitList[index];
          if (this.game.HandyFunctionsObj.CanWeSeeUnit(unit1, this.game.Data.Turn) > 0)
          {
            num12 += 1;
            if (flag2)
            {
              if (num12 <= 3)
              {
                num10 = num5 + 22 + num12 * 84;
                num11 = num6 + 25;
              }
              else
              {
                num10 = num5 + 22 + (num12 - 4) * 84;
                num11 = num6 + 25 + 84;
              }
              bool forcehighlight = false;
              if (this.game.EditObj.UnitSelected == unit1)
                forcehighlight = true;
              this.game.CustomBitmapObj.DrawUnitBig(unit1, forcehighlight, toG, num10, num11, true);
              trect1 = Rectangle::new(num10, num11, 76, 76);
              trect2 = trect1;
              this.AddMouse(ref trect2, "ENEMY UNIT", "An enemy unit that is being targetted", 10000 + unit1);
            }
            else if (num12 < 28)
            {
              if (num12 <= 6)
              {
                num10 = num5 + 22 + num12 * 48;
                num11 = num6 + 25;
              }
              else if (num12 <= 13)
              {
                num10 = num5 + 22 + (num12 - 7) * 48;
                num11 = num6 + 25 + 45;
              }
              else if (num12 <= 20)
              {
                num10 = num5 + 22 + (num12 - 14) * 48;
                num11 = num6 + 25 + 90;
              }
              else
              {
                num10 = num5 + 22 + (num12 - 21) * 48;
                num11 = num6 + 25 + 135;
              }
              bool forcehighlight = false;
              if (this.game.EditObj.UnitSelected == unit1)
                forcehighlight = true;
              this.game.CustomBitmapObj.DrawUnit(unit1, forcehighlight, toG, num10, num11, true);
              trect2 = Rectangle::new(num10, num11, 37, 37);
              trect1 = trect2;
              this.AddMouse(ref trect1, "ENEMY UNIT", "Click to see its troop details.", 10000 + unit1);
            }
          }
        }
      }
      num4 = 0;
      let mut num15: i32 =  64;
      let mut num16: i32 =  30;
      if (this.game.SelectX == -1)
        return;
      ref Graphics local11 = ref toG;
      bitmap4 = BitmapStore.GetBitmap(this.game.SE1_SUPERIMPOSEBACKGROUND);
      ref local12: Bitmap = ref bitmap4;
      let mut x6: i32 =  num15;
      let mut y6: i32 =  num16;
      DrawMod.DrawSimple(ref local11, ref local12, x6, y6);
      ref Graphics local13 = ref toG;
      bitmap4 = BitmapStore.GetBitmap(this.game.SE1_SIDEBARHEADER);
      ref local14: Bitmap = ref bitmap4;
      let mut x7: i32 =  num15 + 120;
      let mut y7: i32 =  num16 - 20;
      DrawMod.DrawSimple(ref local13, ref local14, x7, y7);
      str3: String = "Eligible Forces";
      SizeF sizeF4 = toG.MeasureString(str3, this.game.MarcFont4);
      DrawMod.DrawTextColouredMarc(ref toG, str3, this.game.MarcFont4,  Math.Round( ( (num15 + 120 + 70) - sizeF4.Width / 2f)), num16 - 10, Color.White);
      let mut num17: i32 =  -1;
      let mut counter1: i32 =  this.EL.counter;
      for (let mut index: i32 =  0; index <= counter1; index += 1)
      {
        unit1 = this.EL.unr[index];
        if (!this.game.EditObj.TempUnitList.CheckIfPresent(unit1))
        {
          num17 += 1;
          if (flag2)
          {
            if (num17 <= 3)
            {
              num10 = num15 + 22 + num17 * 84;
              num11 = num16 + 25;
            }
            else
            {
              num10 = num15 + 22 + (num17 - 4) * 84;
              num11 = num16 + 25 + 84;
            }
            bool forcehighlight = false;
            if (this.game.EditObj.UnitSelected == unit1)
              forcehighlight = true;
            this.game.CustomBitmapObj.DrawUnitBig(unit1, forcehighlight, toG, num10, num11, true);
            if (!this.game.Data.UnitObj[unit1].tempCanAttack2)
            {
              trect2 = Rectangle::new(num10, num11, 76, 76);
              trect1 = trect2;
              this.AddMouse(ref trect1, "FRIENDLY UNIT", "Left click to select / de-select in attack participation.", 10000 + unit1);
            }
            else
            {
              DrawMod.DrawBlock(ref toG, num10, num11, 76, 76, 0, 0, 0, 110);
              trect2 = Rectangle::new(num10, num11, 76, 76);
              trect1 = trect2;
              this.AddMouse(ref trect1, "NOT ENOUGH AMMO", "Unit only has " + this.EL.data[index].ToString() + "% of the Ammo needed to participate in this Ranged Attack.");
            }
          }
          else if (num17 < 28)
          {
            if (num17 <= 6)
            {
              num10 = num15 + 22 + num17 * 48;
              num11 = num16 + 25;
            }
            else if (num17 <= 13)
            {
              num10 = num15 + 22 + (num17 - 7) * 48;
              num11 = num16 + 25 + 45;
            }
            else if (num17 <= 20)
            {
              num10 = num15 + 22 + (num17 - 14) * 48;
              num11 = num16 + 25 + 90;
            }
            else
            {
              num10 = num15 + 22 + (num17 - 21) * 48;
              num11 = num16 + 25 + 135;
            }
            bool forcehighlight = false;
            if (this.game.EditObj.UnitSelected == unit1)
              forcehighlight = true;
            this.game.CustomBitmapObj.DrawUnit(unit1, forcehighlight, toG, num10, num11, true);
            trect2 = Rectangle::new(num10, num11, 37, 37);
            trect1 = trect2;
            this.AddMouse(ref trect1, "FRIENDLY UNIT", "Left click to select / de-select in attack participation.", 10000 + unit1);
          }
        }
      }
      let mut num18: i32 =  0;
      let mut num19: i32 =  0;
      let mut num20: i32 =  1;
      do
      {
        num4 = 0;
        let mut x8: i32 =  83;
        if (num20 == 0)
          x8 = 871;
        let mut maxValue: i32 =   byte.MaxValue;
        if (num20 == 1)
          str3 = "Estimation of offensive mods";
        if (num20 == 0)
          str3 = "Estimation of defensive mods";
        DrawMod.DrawTextColouredMarc(ref toG, str3, this.game.MarcFont4, x8, maxValue, Color.White);
        let mut y8: i32 =  maxValue + 25;
        let mut num21: i32 =  1;
        let mut counter2: i32 =  simpleStringList.Counter;
        for (let mut index: i32 =  0; index <= counter2; index += 1)
        {
          if (simpleStringList.Data3[index] == num20 & simpleStringList.Data2[index] != 0 & simpleStringList.Data4[index] > 0)
          {
            let mut num22: i32 =  simpleStringList.Data1[index];
            let mut num23: i32 =  Strings.InStr(simpleStringList.Id[index], "#");
            str4: String = Strings.Left(simpleStringList.Id[index], num23 - 1);
            ttext: String = Strings.Mid(simpleStringList.Id[index], num23 + 1);
            ref Graphics local15 = ref toG;
            bitmap4 = BitmapStore.GetBitmap(this.game.COMBATICONS);
            ref local16: Bitmap = ref bitmap4;
            trect2 = Rectangle::new(32 * (num22 - 1), 0, 32, 32);
            let mut srcrect: &Rectangle = &trect2
            trect1 = Rectangle::new(x8, y8, 32, 32);
            let mut destrect: &Rectangle = &trect1
            DrawMod.DrawSimplePart2(ref local15, ref local16, srcrect, destrect);
            double num24;
            if (num22 >= 25 & num22 <= 28)
            {
              str3 = simpleStringList.Data2[index].ToString();
              if (simpleStringList.Data2[index] >= 10000)
              {
                num24 = Math.Round( simpleStringList.Data2[index] / 1000.0, 1);
                str3 = num24.ToString() + "K".to_owned();
              }
              if (num22 == 26)
                num19 = simpleStringList.Data2[index];
              if (num22 == 28)
                num18 = simpleStringList.Data2[index];
              DrawMod.DrawTextColouredMarc(ref toG, str3, this.game.MarcFont4, x8 + 29, y8 + 6, Color.White);
            }
            else
            {
              str3 = simpleStringList.Data2[index].ToString() + "%";
              if (simpleStringList.Data2[index] >= 10000)
              {
                num24 = Math.Round( simpleStringList.Data2[index] / 1000.0, 1);
                str3 = num24.ToString() + "K%";
              }
              if (simpleStringList.Data2[index] > 0)
                str3 = "+" + str3;
              if (simpleStringList.Data4[index] == 0)
                DrawMod.DrawTextColouredMarc(ref toG, "?", this.game.MarcFont4, x8 + 29, y8 + 6, Color.LightGray);
              else if (simpleStringList.Data2[index] > 0)
                DrawMod.DrawTextColouredMarc(ref toG, str3, this.game.MarcFont4, x8 + 29, y8 + 6, Color.PaleGreen);
              else if (simpleStringList.Data2[index] < 0)
                DrawMod.DrawTextColouredMarc(ref toG, str3, this.game.MarcFont4, x8 + 29, y8 + 6, Color.PaleVioletRed);
              else
                DrawMod.DrawTextColouredMarc(ref toG, str3, this.game.MarcFont4, x8 + 29, y8 + 6, Color.White);
            }
            trect2 = Rectangle::new(x8, y8, 70, 32);
            trect1 = trect2;
            this.AddMouse(ref trect1, str4.ToUpper(), ttext);
            switch (num21)
            {
              case 1:
                x8 += 80;
                num21 = 2;
                continue;
              case 2:
                x8 += 80;
                num21 = 3;
                continue;
              case 3:
                x8 += 80;
                num21 = 4;
                continue;
              default:
                num21 = 1;
                x8 = 83;
                if (num20 == 0)
                  x8 = 871;
                y8 += 32;
                continue;
            }
          }
        }
        num20 += -1;
      }
      while (num20 >= 0);
      str5: String = "odds " + num19.ToString() + " : " + num18.ToString();
      color: Color;
      if (num19 > 0 & num18 < 1)
        str5 = "No return fire expected";
      else if (num19 < 1 | num18 < 1)
        str5 = "unknown odds";
      else if (num19 >= num18)
      {
        float num25 =  Math.Round( num19 /  num18, 1);
        if ( num25 > 5.0)
          num25 =  Math.Round( num25, 0);
        color =  num25 >= 1.5 ? ( num25 >= 2.0 ? ( num25 >= 3.0 ? ( num25 >= 5.0 ? ( num25 >= 10.0 ? Color.FromArgb( byte.MaxValue, 0, 210, 50) : Color.FromArgb( byte.MaxValue, 50, 200, 50)) : Color.FromArgb( byte.MaxValue, 90, 200, 50)) : Color.FromArgb( byte.MaxValue, 130, 200, 50)) : Color.FromArgb( byte.MaxValue, 170, 200, 50)) : Color.FromArgb( byte.MaxValue, 210, 200, 50);
        str5 = "odds  " + num25.ToString() + " : 1";
      }
      else if (num18 > num19)
      {
        float num26 =  Math.Round( num18 /  num19, 1);
        if ( num26 > 5.0)
          num26 =  Math.Round( num26, 0);
        color =  num26 >= 1.5 ? ( num26 >= 2.0 ? ( num26 >= 3.0 ? ( num26 >= 5.0 ? ( num26 >= 10.0 ? Color.FromArgb( byte.MaxValue, 220, 10, 50) : Color.FromArgb( byte.MaxValue, 220, 50, 50)) : Color.FromArgb( byte.MaxValue, 220, 90, 50)) : Color.FromArgb( byte.MaxValue, 220, 130, 50)) : Color.FromArgb( byte.MaxValue, 220, 170, 50)) : Color.FromArgb( byte.MaxValue, 220, 210, 50);
        str5 = "odds  1 : " + num26.ToString();
      }
      DrawMod.DrawBlock(ref toG, 468, 559, 344, 60, 0, 0, 0, 55);
      DrawMod.DrawBlock(ref toG, 468, 559, 344, 60,  color.R,  color.G,  color.B, 105);
      DrawMod.DrawRectangle(ref toG, 468, 559, 344, 60,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 105, 2);
      if ( toG.MeasureString(str5, this.game.introFont2).Width > 300.0)
        DrawMod.DrawTextColouredMarcCenter(ref toG, str5, this.game.introFont1, 640, 571, Color.White);
      else
        DrawMod.DrawTextColouredMarcCenter(ref toG, str5, this.game.introFont2, 640, 564, Color.White);
      num4 = 0;
      let mut num27: i32 =  64;
      let mut num28: i32 =  534;
      if (this.game.SelectX == -1)
        return;
      ref Graphics local17 = ref toG;
      bitmap4 = BitmapStore.GetBitmap(this.game.SE1_SUPERIMPOSEBACKGROUND);
      ref local18: Bitmap = ref bitmap4;
      let mut x9: i32 =  num27;
      let mut y9: i32 =  num28;
      DrawMod.DrawSimple(ref local17, ref local18, x9, y9);
      ref Graphics local19 = ref toG;
      bitmap4 = BitmapStore.GetBitmap(this.game.SE1_SIDEBARHEADER);
      ref local20: Bitmap = ref bitmap4;
      let mut x10: i32 =  num27 + 120;
      let mut y10: i32 =  num28 - 20;
      DrawMod.DrawSimple(ref local19, ref local20, x10, y10);
      str6: String = "Attacking Forces";
      SizeF sizeF5 = toG.MeasureString(str6, this.game.MarcFont4);
      DrawMod.DrawTextColouredMarc(ref toG, str6, this.game.MarcFont4,  Math.Round( ( (num27 + 120 + 70) - sizeF5.Width / 2f)), num28 - 10, Color.White);
      let mut num29: i32 =  -1;
      let mut counter3: i32 =  this.game.EditObj.TempUnitList.counter;
      for (let mut index1: i32 =  0; index1 <= counter3; index1 += 1)
      {
        unit1 = this.game.EditObj.TempUnitList.unr[index1];
        num29 += 1;
        if (flag2)
        {
          if (num29 <= 3)
          {
            num10 = num27 + 22 + num29 * 84;
            num11 = num28 + 25;
          }
          else
          {
            num10 = num27 + 22 + (num29 - 4) * 84;
            num11 = num28 + 25 + 84;
          }
          bool flag4 = false;
          let mut num30: i32 =  100;
          let mut counter4: i32 =  simpleStringList.Counter;
          for (let mut index2: i32 =  0; index2 <= counter4; index2 += 1)
          {
            if (simpleStringList.Data1[index2] == -1 && simpleStringList.Data2[index2] == unit1)
            {
              num30 = simpleStringList.Weight[index2];
              if (num30 < 100)
                flag4 = true;
            }
          }
          bool forcehighlight = false;
          if (this.game.EditObj.UnitSelected == unit1)
            forcehighlight = true;
          if (flag4)
          {
            this.game.CustomBitmapObj.DrawUnitBig(unit1, forcehighlight, toG, num10, num11, true);
            DrawMod.DrawBlock(ref toG, num10, num11, 76, 76, 0, 0, 0, 96);
            DrawMod.DrawBlock(ref toG, num10 + 15, num11 + 10, 46, 20, 0, 0, 0, 128);
            DrawMod.DrawTextColouredMarcCenter(ref toG, num30.ToString() + "%", this.game.MarcFont4, num10 + 15 + 23, num11 + 12, Color.White);
            trect2 = Rectangle::new(num10, num11, 76, 76);
            trect1 = trect2;
            this.AddMouse(ref trect1, "FRIENDLY UNIT (IMPAIRED)", "Not 100% Ammunitions available hence artillery in this Unit will not fire.\r\nAmmunitions avalaible: " + num30.ToString() + "%.\r\nLeft click to select / de-select in attack participation.", 10000 + unit1);
          }
          else
          {
            this.game.CustomBitmapObj.DrawUnitBig(unit1, forcehighlight, toG, num10, num11, true);
            trect2 = Rectangle::new(num10, num11, 76, 76);
            trect1 = trect2;
            this.AddMouse(ref trect1, "FRIENDLY UNIT", "Left click to select / de-select in attack participation.", 10000 + unit1);
          }
        }
        else if (num29 < 28)
        {
          if (num29 <= 6)
          {
            num10 = num27 + 22 + num29 * 48;
            num11 = num28 + 25;
          }
          else if (num29 <= 13)
          {
            num10 = num27 + 22 + (num29 - 7) * 48;
            num11 = num28 + 25 + 45;
          }
          else if (num29 <= 20)
          {
            num10 = num27 + 22 + (num29 - 14) * 48;
            num11 = num28 + 25 + 90;
          }
          else
          {
            num10 = num27 + 22 + (num29 - 21) * 48;
            num11 = num28 + 25 + 135;
          }
          bool forcehighlight = false;
          if (this.game.EditObj.UnitSelected == unit1)
            forcehighlight = true;
          this.game.CustomBitmapObj.DrawUnit(unit1, forcehighlight, toG, num10, num11, true);
        }
        trect2 = Rectangle::new(num10, num11, 37, 37);
        trect1 = trect2;
        this.AddMouse(ref trect1, "FRIENDLY UNIT", "Left click to select / de-select in attack participation.", 10000 + unit1);
      }
      num4 = 0;
      let mut num31: i32 =  842;
      let mut num32: i32 =  30;
      ListClass tListobj = ListClass::new();
      int[] numArray1 = new int[100];
      int[] numArray2 = new int[100];
      tListobj.add("UNITGROUP", -1, "ATTACKER", "DEFENDER");
      let mut num33: i32 =  0;
      let mut Number1: i32 =  0;
      int[] numArray3 = new int[7];
      let mut Index: i32 =  this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].Regime;
      if (Index == this.game.Data.Turn)
        Index = -1;
      let mut counter5: i32 =  this.game.EditObj.TempUnitList.counter;
      num34: i32;
      num35: i32;
      for (let mut index3: i32 =  0; index3 <= counter5; index3 += 1)
      {
        unit1 = this.game.EditObj.TempUnitList.unr[index3];
        let mut num36: i32 =  this.game.HandyFunctionsObj.Distance(this.game.SelectX, this.game.SelectY, 0, this.game.Data.UnitObj[unit1].X, this.game.Data.UnitObj[unit1].Y, 0, 899);
        let mut sfCount: i32 =  this.game.Data.UnitObj[unit1].SFCount;
        for (let mut index4: i32 =  0; index4 <= sfCount; index4 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unit1].SFList[index4];
          if (Index > -1)
          {
            num34 += this.game.Data.SFObj[sf].Qty;
            num35 += this.game.Data.SFObj[sf].Qty * this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[unit1].X, this.game.Data.UnitObj[unit1].Y].get_ReconPts(Index);
          }
          let mut type: i32 =  this.game.Data.SFObj[sf].Type;
          let mut num37: i32 =  this.game.Data.SFObj[sf].Qty * this.game.Data.SFTypeObj[type].Ratio;
          let mut unitGroup: i32 =  this.game.Data.SFTypeObj[type].UnitGroup;
          bool flag5 = true;
          if (this.game.EditObj.OrderType == 11 && this.game.Data.SFTypeObj[type].ArtRange < num36)
            flag5 = false;
          if (this.game.EditObj.OrderType == 14 && !this.game.Data.UnitObj[unit1].tempCanAttack)
            flag5 = false;
          if (this.game.EditObj.OrderType == 33 && !this.game.Data.UnitObj[unit1].tempCanAttack)
            flag5 = false;
          if (flag5)
          {
            int[] numArray4 = numArray1;
            int[] numArray5 = numArray4;
            let mut index5: i32 =  unitGroup;
            let mut index6: i32 =  index5;
            let mut num38: i32 =  numArray4[index5] + num37;
            numArray5[index6] = num38;
            num33 += this.game.Data.SFTypeObj[type].Frontage * this.game.Data.SFObj[sf].Qty;
          }
          let mut index7: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.SelectX, this.game.SelectY, 0, this.game.Data.UnitObj[unit1].X, this.game.Data.UnitObj[unit1].Y, 0);
          if (index7 >= 1 & index7 <= 6)
            numArray3[index7] = 1;
        }
      }
      let mut Number2: i32 =  num34 <= 0 ? 0 :  Math.Round( num35 /  num34);
      Coordinate reconMinusHide1;
      if (this.game.Data.UnitObj[unit1].Regime == this.game.Data.Turn | !this.game.Data.FOWOn | this.game.Data.Round == 0)
        reconMinusHide1.x = 3;
      else
        reconMinusHide1 = this.game.HandyFunctionsObj.GetReconMinusHide(unit1, this.game.Data.Turn);
      let mut Number3: i32 =  this.game.EditObj.OrderType != 11 ? (this.game.EditObj.OrderType != 14 ? (this.game.EditObj.OrderType != 33 ? num33 + this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].get_BattleStack(this.game.Data.Turn) : num33 + this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].get_BattleStackAir(this.game.Data.Turn)) : num33 + this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].get_BattleStackAir(this.game.Data.Turn)) : num33 + this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].get_BattleStackArt(this.game.Data.Turn);
      bool flag6 = false;
      let mut unitCounter: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter;
      for (let mut index8: i32 =  0; index8 <= unitCounter; index8 += 1)
      {
        let mut unit2: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitList[index8];
        if (this.game.HandyFunctionsObj.CanWeSeeUnit(unit2, this.game.Data.Turn) > 0)
        {
          let mut sfCount: i32 =  this.game.Data.UnitObj[unit2].SFCount;
          for (let mut i: i32 =  0; i <= sfCount; i += 1)
          {
            let mut sf: i32 =  this.game.Data.UnitObj[unit2].SFList[i];
            let mut type: i32 =  this.game.Data.SFObj[sf].Type;
            let mut num39: i32 =  this.game.Data.SFObj[sf].Qty * this.game.Data.SFTypeObj[type].Ratio;
            let mut unitGroup: i32 =  this.game.Data.SFTypeObj[type].UnitGroup;
            Coordinate reconMinusHide2 = this.game.HandyFunctionsObj.GetReconMinusHide(unit2, this.game.Data.Turn);
            if (reconMinusHide2.x > 1)
            {
              num40: i32;
              if (reconMinusHide2.x < 3 && this.game.Data.FOWOn & this.game.Data.UnitObj[unit2].Regime != this.game.Data.Turn && reconMinusHide2.x == 2)
              {
                let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
                this.game.HandyFunctionsObj.RandomizeForUnit(unit2, i);
                float num41 =  reconMinusHide2.y / (this.game.Data.RuleVar[56] - this.game.Data.RuleVar[55]);
                float num42 =  ((1.0 -  num41) * 2.0);
                num40 =  Math.Round( ((VBMath.Rnd() * num42 + num41) *  qty));
                if (num40 < 1)
                  num40 = 1;
                num39 = num40 * this.game.Data.SFTypeObj[type].Ratio;
                VBMath.Randomize();
              }
              int[] numArray6 = numArray2;
              int[] numArray7 = numArray6;
              let mut index9: i32 =  unitGroup;
              let mut index10: i32 =  index9;
              let mut num43: i32 =  numArray6[index9] + num39;
              numArray7[index10] = num43;
              Number1 += this.game.Data.SFTypeObj[type].Frontage * num40;
            }
            else
              flag6 = true;
          }
        }
      }
      let mut num44: i32 =  0;
      let mut index11: i32 =  0;
      do
      {
        if (numArray1[index11] > 0 | numArray2[index11] > 0)
        {
          num44 += 1;
          if (numArray2[index11] == 0 & flag6)
            tListobj.add(this.game.Data.TempString[400 + index11], -1, Strings.Trim(Conversion.Str( numArray1[index11])), "?");
          else
            tListobj.add(this.game.Data.TempString[400 + index11], -1, Strings.Trim(Conversion.Str( numArray1[index11])), Strings.Trim(Conversion.Str( numArray2[index11])));
        }
        index11 += 1;
      }
      while (index11 <= 99);
      if (num44 == 0)
      {
        let mut index12: i32 =  0;
        tListobj.add(this.game.Data.TempString[400 + index12], -1, Strings.Trim(Conversion.Str( numArray1[index12])), "?");
      }
      let mut num45: i32 =  4;
      let mut Number4: i32 =  0;
      let mut Number5: i32 =  0;
      if (tListobj.ListCount > num45)
      {
        let mut num46: i32 =  num45 + 1;
        let mut listCount: i32 =  tListobj.ListCount;
        for (let mut index13: i32 =  num46; index13 <= listCount; index13 += 1)
        {
          Number4 +=  Math.Round(Conversion.Val(tListobj.ListValue[index13]));
          Number5 +=  Math.Round(Conversion.Val(tListobj.ListValue2[index13]));
        }
        tListobj.ListCount = num45;
        tListobj.add("Other", -1, Strings.Trim(Conversion.Str( Number4)), Strings.Trim(Conversion.Str( Number5)));
      }
      let mut num47: i32 =  0;
      let mut index14: i32 =  1;
      do
      {
        if (numArray3[index14] > 0)
          num47 += 1;
        index14 += 1;
      }
      while (index14 <= 6);
      if (num47 < 2)
        num47 = 2;
      let mut Number6: i32 =   Math.Round( (this.game.Data.RuleVar[31] *  num47));
      let mut Number7: i32 =   Math.Round( this.game.Data.RuleVar[30]);
      if (this.game.EditObj.OrderType == 11)
        Number6 =  Math.Round( this.game.Data.RuleVar[834]);
      if (this.game.EditObj.OrderType == 14)
        Number6 =  Math.Round( this.game.Data.RuleVar[833]);
      if (this.game.EditObj.OrderType == 33)
        Number6 =  Math.Round( this.game.Data.RuleVar[833]);
      if (numArray2[index14] == 0 & flag6)
        tListobj.add("Stack Points", -1, Strings.Trim(Conversion.Str( Number3)) + " / " + Strings.Trim(Conversion.Str( Number6)), "? / " + Strings.Trim(Conversion.Str( Number7)));
      else
        tListobj.add("Stack Points", -1, Strings.Trim(Conversion.Str( Number3)) + " / " + Strings.Trim(Conversion.Str( Number6)), Strings.Trim(Conversion.Str( Number1)) + " / " + Strings.Trim(Conversion.Str( Number7)));
      tListobj.add("Recon Points", -1, Strings.Trim(Conversion.Str( this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].MaxRecon)) + " / " + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[56])), Strings.Trim(Conversion.Str( Number2)) + " / " + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[56])));
      ref Graphics local21 = ref toG;
      bitmap4 = BitmapStore.GetBitmap(this.game.SE1_SUPERIMPOSEBACKGROUND);
      ref local22: Bitmap = ref bitmap4;
      let mut x11: i32 =  num31;
      let mut y11: i32 =  num32;
      DrawMod.DrawSimple(ref local21, ref local22, x11, y11);
      ref Graphics local23 = ref toG;
      bitmap4 = BitmapStore.GetBitmap(this.game.SE1_SIDEBARHEADER);
      ref local24: Bitmap = ref bitmap4;
      let mut x12: i32 =  num31 + 120;
      let mut y12: i32 =  num32 - 20;
      DrawMod.DrawSimple(ref local23, ref local24, x12, y12);
      str7: String = "Combat Totals";
      SizeF sizeF6 = toG.MeasureString(str7, this.game.MarcFont4);
      DrawMod.DrawTextColouredMarc(ref toG, str7, this.game.MarcFont4,  Math.Round( ( (num31 + 120 + 70) - sizeF6.Width / 2f)), num32 - 10, Color.White);
      let mut num48: i32 =  7;
      ListSubPartClass listSubPartClass = new ListSubPartClass(tListobj, num48 + 1, 330, -1, this.game, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 200, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: (num31 + 20), bby: (num32 + 25), tMarcStyle: true, overruleFont: (ref this.game.MarcFont4), overruleItemSize: 18);
      ref Graphics local25 = ref toG;
      bitmap4 = listSubPartClass.Paint();
      ref local26: Bitmap = ref bitmap4;
      let mut x13: i32 =  num31 + 20;
      let mut y13: i32 =  num32 + 25;
      DrawMod.DrawSimple(ref local25, ref local26, x13, y13);
      buttontext: String = "ATTACK".to_owned();
      tDescript1: String = "Click to start the attack [Space]";
      tDescript2: String = "Select some attackers in order to be able to attack [Space]";
      if (this.game.EditObj.OrderType == 11)
      {
        buttontext = "RANGED ATTACK";
        tDescript1 = "Click to start the ranged attack [Space]";
        tDescript2 = "Select some ranged-fire capable attackers in order to be able to ranged attack [Space]";
      }
      if (this.game.EditObj.OrderType == 14)
      {
        buttontext = "AIR ATTACK";
        tDescript1 = "Click to start the air attack [Space]";
        tDescript2 = "Select some air units to join in this air attack [Space]";
      }
      if (this.game.EditObj.OrderType == 33)
      {
        buttontext = "AIR RECON";
        tDescript1 = "Click to start the air recon [Space]";
        tDescript2 = "Select some air units to join in this air recon [Space]";
      }
      if (this.game.EditObj.TempUnitList.counter > -1)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(buttontext, 350, tDescript1, ref this.OwnBitmap, 468, 631, tred: true, theight: 85, usefont: this.game.MarcFont1, useshadow: true, tMarcStyle: true);
        this.okid = this.AddSubPart(ref tsubpart, 468, 631, 350, 85, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(buttontext, 350, tDescript2, ref this.OwnBitmap, 468, 631, true, true, 85, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.okid = this.AddSubPart(ref tsubpart, 468, 631, 350, 85, 1);
      }
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Cancel", 150, "Click to return to main screen [Esc]", ref this.OwnBitmap, 680, 142, theight: 65, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
      this.cancelid = this.AddSubPart(ref tsubpart1, 680, 142, 150, 65, 1);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Switch", 150, "Switch between Regular Attack and Ranged Attack preperation", ref this.OwnBitmap, 450, 142, theight: 65, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
      this.switchId = this.AddSubPart(ref tsubpart2, 450, 142, 150, 65, 1);
      let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Map", 70, "Select elgible Units on the Map instead.", ref this.OwnBitmap, 605, 142, theight: 65, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
      this.mapButId = this.AddSubPart(ref tsubpart3, 605, 142, 70, 65, 1);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27)
        {
          let mut unitCounter: i32 =  this.game.Data.UnitCounter;
          for (let mut index: i32 =  0; index <= unitCounter; index += 1)
            this.game.Data.UnitObj[index].tempCanAttack = false;
          this.game.EditObj.HideUnit = this.oldMode1;
          this.game.EditObj.SpreadUnit = (uint) this.oldMode2 > 0U;
          this.game.EditObj.TempUnitList = UnitList::new();
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if (nr == 32 & this.game.EditObj.TempUnitList.counter > -1)
        {
          let mut unitCounter: i32 =  this.game.Data.UnitCounter;
          for (let mut index: i32 =  0; index <= unitCounter; index += 1)
            this.game.Data.UnitObj[index].tempCanAttack = false;
          this.game.EditObj.HideUnit = this.oldMode1;
          this.game.EditObj.SpreadUnit = (uint) this.oldMode2 > 0U;
          this.game.EditObj.TargetX = this.game.SelectX;
          this.game.EditObj.TargetY = this.game.SelectY;
          this.game.EditObj.OrderX = this.game.SelectX;
          this.game.EditObj.OrderY = this.game.SelectY;
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

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut num1: i32 =  574;
      let mut num2: i32 =  327;
      Coordinate target;
      target.x = this.game.SelectX;
      target.y = this.game.SelectY;
      target.onmap = true;
      let mut tfacing: i32 =  1;
      do
      {
        Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, 0, tfacing);
        if (coordinate.onmap)
        {
          let mut num3: i32 =  num1;
          let mut num4: i32 =  num2;
          if (tfacing == 1)
            num4 -= 96;
          if (tfacing == 2)
          {
            num3 += 106;
            num4 -= 48;
          }
          if (tfacing == 3)
          {
            num3 += 106;
            num4 += 48;
          }
          if (tfacing == 4)
            num4 += 96;
          if (tfacing == 5)
          {
            num3 -= 106;
            num4 += 48;
          }
          if (tfacing == 6)
          {
            num3 -= 106;
            num4 -= 48;
          }
          if (x >= num3 & x < num3 + 128 && y >= num4 & y < num4 + 96 && this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime == this.game.Data.Turn)
          {
            let mut zoom: i32 =  this.game.EditObj.Zoom;
            this.game.EditObj.Zoom = 1;
            let mut num5: i32 =  this.game.HandyFunctionsObj.ClickOnHexGivesUnit(coordinate.x, coordinate.y, this.game.EditObj.MapSelected, false, b, x - num3, y - num4);
            this.game.EditObj.Zoom = zoom;
            if (num5 > -1)
            {
              if (this.game.EditObj.TempUnitList.CheckIfPresent(num5))
              {
                this.game.EditObj.TempUnitList.remove(num5);
                this.View();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num5 > -1)
              {
                if ( this.game.Data.RuleVar[307] <=  this.game.HandyFunctionsObj.GetPowerPtsAbsolute(num5))
                {
                  if (!this.game.HandyFunctionsObj.CanDoLandAttack(num5, target))
                    num5 = -1;
                }
                else
                  num5 = -1;
              }
              if (num5 > -1)
              {
                this.game.EditObj.TempUnitList.add(num5);
                this.View();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
          }
        }
        tfacing += 1;
      }
      while (tfacing <= 6);
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (this.MouseData[index] > 0 && x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height && this.MouseData[index] >= 0)
        {
          let mut tunr: i32 =  this.MouseData[index] - 10000;
          if (this.game.Data.UnitObj[tunr].Regime == this.game.Data.Turn)
          {
            if (!this.game.EditObj.TempUnitList.CheckIfPresent(tunr))
              this.game.EditObj.TempUnitList.add(tunr);
            else
              this.game.EditObj.TempUnitList.remove(tunr);
            this.View();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num6: i32 =  this.SubPartID[index1];
            if (num6 == this.okid)
            {
              let mut unitCounter: i32 =  this.game.Data.UnitCounter;
              for (let mut index2: i32 =  0; index2 <= unitCounter; index2 += 1)
                this.game.Data.UnitObj[index2].tempCanAttack = false;
              this.game.EditObj.HideUnit = this.oldMode1;
              this.game.EditObj.SpreadUnit = (uint) this.oldMode2 > 0U;
              this.game.EditObj.TargetX = this.game.SelectX;
              this.game.EditObj.TargetY = this.game.SelectY;
              this.game.EditObj.OrderX = this.game.SelectX;
              this.game.EditObj.OrderY = this.game.SelectY;
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num6 == this.mapButId)
            {
              this.game.EditObj.PopupValue = 30;
              this.game.EditObj.HideUnit = this.oldMode1;
              this.game.EditObj.SpreadUnit = (uint) this.oldMode2 > 0U;
              let mut unitCounter: i32 =  this.game.Data.UnitCounter;
              for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
                this.game.Data.UnitObj[index3].TempUnitSelectable = false;
              let mut counter: i32 =  this.EL.counter;
              for (let mut index4: i32 =  0; index4 <= counter; index4 += 1)
                this.game.Data.UnitObj[this.EL.unr[index4]].TempUnitSelectable = true;
              windowReturnClass.AddCommand(5, 14);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num6 == this.cancelid)
            {
              let mut unitCounter: i32 =  this.game.Data.UnitCounter;
              for (let mut index5: i32 =  0; index5 <= unitCounter; index5 += 1)
                this.game.Data.UnitObj[index5].tempCanAttack = false;
              this.game.EditObj.HideUnit = this.oldMode1;
              this.game.EditObj.SpreadUnit = (uint) this.oldMode2 > 0U;
              this.game.EditObj.TempUnitList = UnitList::new();
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num6 == this.switchId)
            {
              this.game.EditObj.TempUnitList = UnitList::new();
              if (this.game.EventRelatedObj.Helper_AirEnabled())
              {
                if (this.game.EditObj.OrderType == 2)
                  this.game.EditObj.OrderType = 11;
                else if (this.game.EditObj.OrderType == 11)
                  this.game.EditObj.OrderType = 14;
                else if (this.game.EditObj.OrderType == 14)
                  this.game.EditObj.OrderType = 33;
                else if (this.game.EditObj.OrderType == 33)
                  this.game.EditObj.OrderType = 2;
              }
              else if (this.game.EditObj.OrderType == 2)
                this.game.EditObj.OrderType = 11;
              else if (this.game.EditObj.OrderType == 11)
                this.game.EditObj.OrderType = 2;
              this.Setup();
              this.View();
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
