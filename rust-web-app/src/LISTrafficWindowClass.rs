// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LISTrafficWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Imaging;

namespace WindowsApplication1
{
  pub class LISTrafficWindowClass : WindowClass
  {
     okid: i32;
     trafficId: Vec<i32>;
     int[] trafficIdB;
     int[] pullId;
     int[] pullData;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
     clearAllid: i32;
     clearAllidB: i32;
     clearAll2id: i32;
     clearAll2idB: i32;
     flag1id: i32;
     flag2id: i32;
     flag3id: i32;
     log1id: i32;
     log2id: i32;
     log3id: i32;
     log1Bid: i32;
     log2Bid: i32;
     log3Bid: i32;
     logType1id: i32;
     logType2id: i32;
     logType1bid: i32;
     logType2bid: i32;
     overruleId: i32;
     overruleIdB: i32;
     overruleId2: i32;
     overruleId2B: i32;
     flagRailId: i32;
     flagTruckId: i32;
     flagPullId: i32;
     logSourceX: i32;
     logSourceY: i32;
     logSourceType: i32;
     SimpleList LogSourceList;

    pub LISTrafficWindowClass( tGame: GameClass)
      : base( tGame, 1200, 768, 8)
    {
      self.trafficId = new int[7, 11];
      self.trafficIdB = new int[7];
      self.pullId = new int[16];
      self.pullData = new int[16];
      self.game.EditObj.layerLis_TraficWindowOpen = 1;
      self.Setup();
      self.View();
    }

    pub fn Setup()
    {
    }

    pub fn Dispose()
    {
      base.Dispose();
      DrawMod.TGame.EditObj.layerLis_TraficWindowOpen = 0;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  self.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            self.SubPartList[index].DescriptInfo(x - self.SubPartX[index], y - self.SubPartY[index]);
            if (Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
            {
              self.game.EditObj.TipButton = true;
              self.game.EditObj.TipTitle = "";
              self.game.EditObj.TipText = self.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 =  self.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 0)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          break;
        }
      }
    }

    pub fn View()
    {
      libName: String = "SE_Data".to_owned();
      let mut stringListById1: i32 =  self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 148, 0, 0));
      let mut stringListById2: i32 =  self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 144, 0, 0));
      let mut stringListById3: i32 =  self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[405]));
      let mut stringListById4: i32 =  self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 210, 0, 0));
      if (self.okid > 0)
      {
        self.RemoveSubPart(self.okid);
        self.okid = 0;
      }
      if (self.log1id > 0)
      {
        self.RemoveSubPart(self.log1id);
        self.log1id = 0;
      }
      if (self.logType1id > 0)
      {
        self.RemoveSubPart(self.logType1id);
        self.logType1id = 0;
      }
      if (self.logType2id > 0)
      {
        self.RemoveSubPart(self.logType2id);
        self.logType2id = 0;
      }
      if (self.logType1bid > 0)
      {
        self.RemoveSubPart(self.logType1bid);
        self.logType1bid = 0;
      }
      if (self.logType2bid > 0)
      {
        self.RemoveSubPart(self.logType2bid);
        self.logType2bid = 0;
      }
      if (self.log2id > 0)
      {
        self.RemoveSubPart(self.log2id);
        self.log2id = 0;
      }
      if (self.log3id > 0)
      {
        self.RemoveSubPart(self.log3id);
        self.log3id = 0;
      }
      if (self.log1Bid > 0)
      {
        self.RemoveSubPart(self.log1Bid);
        self.log1Bid = 0;
      }
      if (self.log2Bid > 0)
      {
        self.RemoveSubPart(self.log2Bid);
        self.log2Bid = 0;
      }
      if (self.log3Bid > 0)
      {
        self.RemoveSubPart(self.log3Bid);
        self.log3Bid = 0;
      }
      if (self.clearAllid > 0)
      {
        self.RemoveSubPart(self.clearAllid);
        self.clearAllid = 0;
      }
      if (self.clearAllidB > 0)
      {
        self.RemoveSubPart(self.clearAllidB);
        self.clearAllidB = 0;
      }
      if (self.clearAll2id > 0)
      {
        self.RemoveSubPart(self.clearAll2id);
        self.clearAll2id = 0;
      }
      if (self.clearAll2idB > 0)
      {
        self.RemoveSubPart(self.clearAll2idB);
        self.clearAll2idB = 0;
      }
      if (self.flag1id > 0)
      {
        self.RemoveSubPart(self.flag1id);
        self.flag1id = 0;
      }
      if (self.flag2id > 0)
      {
        self.RemoveSubPart(self.flag2id);
        self.flag2id = 0;
      }
      if (self.flag3id > 0)
      {
        self.RemoveSubPart(self.flag3id);
        self.flag3id = 0;
      }
      if (self.overruleId > 0)
      {
        self.RemoveSubPart(self.overruleId);
        self.overruleId = 0;
      }
      if (self.overruleIdB > 0)
      {
        self.RemoveSubPart(self.overruleIdB);
        self.overruleIdB = 0;
      }
      if (self.overruleId2 > 0)
      {
        self.RemoveSubPart(self.overruleId2);
        self.overruleId2 = 0;
      }
      if (self.overruleId2B > 0)
      {
        self.RemoveSubPart(self.overruleId2B);
        self.overruleId2B = 0;
      }
      if (self.flagTruckId > 0)
      {
        self.RemoveSubPart(self.flagTruckId);
        self.flagTruckId = 0;
      }
      if (self.flagRailId > 0)
      {
        self.RemoveSubPart(self.flagRailId);
        self.flagRailId = 0;
      }
      if (self.flagPullId > 0)
      {
        self.RemoveSubPart(self.flagPullId);
        self.flagPullId = 0;
      }
      let mut index1: i32 =  0;
      index2: i32;
      do
      {
        if (self.trafficIdB[index1] > 0)
        {
          self.RemoveSubPart(self.trafficIdB[index1]);
          self.trafficIdB[index1] = 0;
        }
        index2 = 0;
        do
        {
          if (self.trafficId[index1, index2] > 0)
          {
            self.RemoveSubPart(self.trafficId[index1, index2]);
            self.trafficId[index1, index2] = 0;
          }
          index2 += 1;
        }
        while (index2 <= 9);
        index1 += 1;
      }
      while (index1 <= 5);
      let mut index3: i32 =  0;
      do
      {
        if (self.pullId[index3] > 0)
        {
          self.RemoveSubPart(self.pullId[index3]);
          self.pullId[index3] = 0;
        }
        index3 += 1;
      }
      while (index3 <= 15);
      self.ClearMouse();
      self.NewBackGroundAndClearAll(1200, 768, 8);
      Graphics graphics1 = Graphics.FromImage((Image) self.OwnBitmap);
      graphics1.Clear(Color.Transparent);
      DrawMod.DrawMessFrame( self.OwnBitmap,  graphics1, 0, 0, 1200, 768);
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Back", 160, "Click to return to main screen [Esc]",  self.OwnBitmap, 820, 680, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart1, 820, 680, 160, 40, 1);
      str1: String = "Blocking % Traffic Signs for " + self.game.SelectX.ToString() + "," + self.game.SelectY.ToString();
      DrawMod.DrawBlock( graphics1, 30, 20, 560, 700, 0, 0, 0, 95);
      SizeF sizeF1 = SizeF::new();
      SizeF sizeF2 = graphics1.MeasureString(str1, self.game.MarcFont2);
      DrawMod.DrawTextColouredMarc( graphics1, str1, self.game.MarcFont3,  Math.Round(365.0 -  sizeF2.Width / 2.0), 35, Color.White);
      DrawMod.DrawBlock( graphics1, 40, 70, 540, 40, 0, 0, 0, 80);
      tstring1: String = "Traffic Signs for:";
      DrawMod.DrawTextColouredMarc( graphics1, tstring1, self.game.MarcFont4, 50, 81, Color.White);
      str2: String = "Logistics Logs for " + self.game.SelectX.ToString() + "," + self.game.SelectY.ToString();
      DrawMod.DrawBlock( graphics1, 600, 20, 560, 640, 0, 0, 0, 95);
      sizeF2 = graphics1.MeasureString(str2, self.game.MarcFont2);
      DrawMod.DrawTextColouredMarc( graphics1, str2, self.game.MarcFont3,  Math.Round(915.0 -  sizeF2.Width / 2.0), 35, Color.White);
      NeighboursExtra lisTraffic = self.game.HandyFunctionsObj.GetLisTraffic(self.game.SelectX, self.game.SelectY);
      let mut x1: i32 =  160;
      let mut y1: i32 =  82;
      tDescript1: String = "Click to toggle on/off the use of Traffics Signs for specific usages. These settings are Hex-wide and apply to all directions.";
      if (lisTraffic.truck)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(self.game.BUTTONFLAGGED, tDescript: tDescript1);
        self.flagTruckId = self.AddSubPart( tsubpart2, x1, y1, 32, 16, 1);
      }
      else
      {
        let mut tsubpart3: SubPartClass =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED, tDescript: tDescript1);
        self.flagTruckId = self.AddSubPart( tsubpart3, x1, y1, 32, 16, 1);
      }
      tstring2: String = "Truck Logistics";
      DrawMod.DrawTextColouredMarc( graphics1, tstring2, self.game.MarcFont4, x1 + 44, y1 - 2, Color.White);
      let mut x2: i32 =  x1 + 150;
      tDescript2: String = "Click to toggle on/off the use of Traffics Signs for specific usages. These settings are Hex-wide and apply to all directions.";
      if (lisTraffic.rail)
      {
        let mut tsubpart4: SubPartClass =  ButtonPartClass::new(self.game.BUTTONFLAGGED, tDescript: tDescript2);
        self.flagRailId = self.AddSubPart( tsubpart4, x2, y1, 32, 16, 1);
      }
      else
      {
        let mut tsubpart5: SubPartClass =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED, tDescript: tDescript2);
        self.flagRailId = self.AddSubPart( tsubpart5, x2, y1, 32, 16, 1);
      }
      tstring3: String = "Rail Logistics";
      DrawMod.DrawTextColouredMarc( graphics1, tstring3, self.game.MarcFont4, x2 + 44, y1 - 2, Color.White);
      let mut x3: i32 =  x2 + 140;
      tDescript3: String = "Click to toggle on/off the use of Traffics Signs for specific usages. These settings are Hex-wide and apply to all directions.";
      if (lisTraffic.pull)
      {
        let mut tsubpart6: SubPartClass =  ButtonPartClass::new(self.game.BUTTONFLAGGED, tDescript: tDescript3);
        self.flagPullId = self.AddSubPart( tsubpart6, x3, y1, 32, 16, 1);
      }
      else
      {
        let mut tsubpart7: SubPartClass =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED, tDescript: tDescript3);
        self.flagPullId = self.AddSubPart( tsubpart7, x3, y1, 32, 16, 1);
      }
      tstring4: String = "Pull Points";
      DrawMod.DrawTextColouredMarc( graphics1, tstring4, self.game.MarcFont4, x3 + 44, y1 - 2, Color.White);
      let mut x4: i32 =  182;
      let mut y2: i32 =  200;
      let mut num1: i32 =  self.game.SelectX;
      let mut selectY: i32 =  self.game.SelectY;
      let mut hideUnit: i32 =  self.game.EditObj.HideUnit;
      bool showLabel = self.game.EditObj.ShowLabel;
      bool layerLisAvailable = self.game.EditObj.layerLisAvailable;
      bool layerLisUsed = self.game.EditObj.layerLisUsed;
      bool layerLisTotal = self.game.EditObj.layerLisTotal;
      bool layerLisBottlenecks = self.game.EditObj.layerLisBottlenecks;
      self.game.EditObj.HideUnit = 0;
      self.game.EditObj.ShowLabel = false;
      self.game.EditObj.layerLisAvailable = false;
      self.game.EditObj.layerLisUsed = false;
      self.game.EditObj.layerLisTotal = true;
      self.game.EditObj.layerLisBottlenecks = false;
      self.game.SelectX = -1;
      self.game.SelectY = -1;
      objBitmap: Bitmap = new Bitmap(128, 96, PixelFormat.Format32bppPArgb);
      objBitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics2 = Graphics.FromImage((Image) objBitmap);
      customBitmapObj: CustomBitmapClass = self.game.CustomBitmapObj;
      let mut cx: i32 =  num1;
      let mut cy: i32 =  selectY;
      Graphics tempg = graphics2;
      bitmap: Bitmap = (Bitmap) null;
       let mut local1: &Bitmap = &bitmap;
      bool flag1 = false;
       bool local2 =  flag1;
      customBitmapObj.DrawHex(cx, cy, 0, tempg: tempg, counteralpha: 0, Zoom: 1, neverusehistory: true, gBitmap: ( local1), tFromMapPopup: ( local2));
      DrawMod.DrawScaled( graphics1,  objBitmap, x4, y2, 256, 192, true);
      objBitmap.Dispose();
      graphics2.Dispose();
      self.game.SelectX = num1;
      self.game.SelectY = selectY;
      self.game.EditObj.HideUnit = hideUnit;
      self.game.EditObj.ShowLabel = showLabel;
      self.game.EditObj.layerLisAvailable = layerLisAvailable;
      self.game.EditObj.layerLisUsed = layerLisUsed;
      self.game.EditObj.layerLisTotal = layerLisTotal;
      self.game.EditObj.layerLisBottlenecks = layerLisBottlenecks;
      let mut num2: i32 =  0;
      let mut num3: i32 =  0;
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index4: i32 =  0; index4 <= mapWidth; index4 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
        {
          if (self.game.Data.RegimeObj[self.game.Data.Turn].Trafic[0].Value[index4, index5] > 1000000)
            num2 += 1;
          if (self.game.Data.RegimeObj[self.game.Data.Turn].Trafic2[0].Value[index4, index5] != 0)
            num3 += 1;
        }
      }
      DrawMod.DrawBlock( graphics1, x4 + 218, 500, 180, 210, 0, 0, 0, 80);
      tstring5: String = "Global Ops";
      DrawMod.DrawTextColouredMarcCenter( graphics1, tstring5, self.game.MarcFont4, x4 + 218 + 90, 513, Color.White);
      let mut num4: i32 =  y2;
      let mut num5: i32 =  540;
      SubPartClass tsubpart8;
      if (num2 > 0)
      {
        let mut tsubpart9: SubPartClass =  new TextButtonPartClass("Clear all " + num2.ToString() + " Signs", 160, "Click to clear all " + num2.ToString() + " Traffic Signs on the whole map.",  self.OwnBitmap, x4 + 228, num5, theight: 32, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.clearAllid = self.AddSubPart( tsubpart9, x4 + 228, num5, 160, 32, 1);
      }
      else
      {
        tsubpart8 =  new TextButtonPartClass("Clear all Signs", 160, "You cannot clear all Traffic Signs because you do not have any Traffic Signs placed on the map.",  self.OwnBitmap, x4 + 228, num5, true, theight: 32, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.clearAllid = self.AddSubPart( tsubpart8, x4 + 228, num5, 160, 32, 0);
      }
      let mut num6: i32 =  num5 + 32 + 5;
      if ( Math.Round(Conversion.Val(self.game.Data.Designer)) >= 42 & !self.game.Data.RegimeObj[self.game.Data.Turn].AI & !self.game.Data.RegimeObj[self.game.Data.Turn].minimumDataUsage)
      {
        if (num3 > 0)
        {
          tsubpart8 =  new TextButtonPartClass("Clear " + num3.ToString() + " Custom Pulls", 160, "Click to clear all " + num3.ToString() + " Custom Pull Points on the whole map.",  self.OwnBitmap, x4 + 228, num6, theight: 32, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.clearAll2id = self.AddSubPart( tsubpart8, x4 + 228, num6, 160, 32, 1);
        }
        else
        {
          tsubpart8 =  new TextButtonPartClass("Clear all Custom Pulls", 160, "You cannot clear Custom Pull Points because you have not placed any.",  self.OwnBitmap, x4 + 228, num6, true, theight: 32, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.clearAll2id = self.AddSubPart( tsubpart8, x4 + 228, num6, 160, 32, 0);
        }
        let mut y3: i32 =  num6 + 32 + 4 + 5;
        tDescript4: String = "Click to toggle on/off the use of Automatic Asset Pull Points. Red Flagged means its toggled on. White means its toggled off.";
        if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "pullAssetsOff", 2))) < 1)
        {
          tsubpart8 =  ButtonPartClass::new(self.game.BUTTONFLAGGED, tDescript: tDescript4);
          self.flag1id = self.AddSubPart( tsubpart8, x4 + 228, y3, 32, 16, 1);
        }
        else
        {
          tsubpart8 =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED, tDescript: tDescript4);
          self.flag1id = self.AddSubPart( tsubpart8, x4 + 228, y3, 32, 16, 1);
        }
        tstring6: String = "Asset Pull Pts";
        DrawMod.DrawTextColouredMarc( graphics1, tstring6, self.game.MarcFont4, x4 + 228 + 40, y3, Color.White);
        let mut y4: i32 =  y3 + 20 + 5;
        tDescript5: String = "Click to toggle on/off the use of Automatic Unit Pull Points. Red Flagged means its toggled on. White means its toggled off.";
        if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "pullUnitsOff", 2))) < 1)
        {
          tsubpart8 =  ButtonPartClass::new(self.game.BUTTONFLAGGED, tDescript: tDescript5);
          self.flag2id = self.AddSubPart( tsubpart8, x4 + 228, y4, 32, 16, 1);
        }
        else
        {
          tsubpart8 =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED, tDescript: tDescript5);
          self.flag2id = self.AddSubPart( tsubpart8, x4 + 228, y4, 32, 16, 1);
        }
        tstring7: String = "Unit Pull Pts";
        DrawMod.DrawTextColouredMarc( graphics1, tstring7, self.game.MarcFont4, x4 + 228 + 40, y4, Color.White);
        let mut y5: i32 =  y4 + 20 + 5;
        tDescript6: String = "Click to toggle on/off the use of Automatic Unit Pull Points. Red Flagged means its toggled on. White means its toggled off.\"";
        num1 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "pullCitiesOff", 2)));
        if (num1 < 1)
        {
          tsubpart8 =  ButtonPartClass::new(self.game.BUTTONFLAGGED, tDescript: tDescript6);
          self.flag3id = self.AddSubPart( tsubpart8, x4 + 228, y5, 32, 16, 1);
        }
        else
        {
          tsubpart8 =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED, tDescript: tDescript6);
          self.flag3id = self.AddSubPart( tsubpart8, x4 + 228, y5, 32, 16, 1);
        }
        tstring8: String = "City Pull Pts";
        DrawMod.DrawTextColouredMarc( graphics1, tstring8, self.game.MarcFont4, x4 + 228 + 40, y5, Color.White);
      }
      let mut num7: i32 =  num4;
      DrawMod.DrawBlock( graphics1, 40, 500, 340, 210, 0, 0, 0, 80);
      str3: String = "Custom Pull Points";
      DrawMod.DrawTextColouredMarcCenter( graphics1, str3, self.game.MarcFont4, 210, 513, Color.White);
      let mut num8: i32 =  50;
      let mut y1_1: i32 =  540;
      color: Color;
      if ( Math.Round(Conversion.Val(self.game.Data.Designer)) >= 42 & !self.game.Data.RegimeObj[self.game.Data.Turn].AI & !self.game.Data.RegimeObj[self.game.Data.Turn].minimumDataUsage)
      {
        index2 = 0;
        do
        {
          if (index2 == 0)
          {
            str3 = "Block".to_owned();
            num1 = -1;
          }
          if (index2 == 1)
          {
            str3 = "None".to_owned();
            num1 = 0;
          }
          if (index2 == 2)
          {
            str3 = "50".to_owned();
            num1 = 50;
          }
          if (index2 == 3)
          {
            str3 = "100".to_owned();
            num1 = 100;
          }
          if (index2 == 4)
          {
            str3 = "200".to_owned();
            num1 = 200;
          }
          if (index2 == 5)
          {
            str3 = "500".to_owned();
            num1 = 500;
          }
          if (index2 == 6)
          {
            str3 = "1K".to_owned();
            num1 = 1000;
          }
          if (index2 == 7)
          {
            str3 = "2K".to_owned();
            num1 = 2000;
          }
          if (index2 == 8)
          {
            str3 = "3K".to_owned();
            num1 = 3000;
          }
          if (index2 == 9)
          {
            str3 = "5K".to_owned();
            num1 = 5000;
          }
          if (index2 == 10)
          {
            str3 = "8K".to_owned();
            num1 = 8000;
          }
          if (index2 == 11)
          {
            str3 = "12K".to_owned();
            num1 = 12000;
          }
          if (index2 == 12)
          {
            str3 = "20K".to_owned();
            num1 = 20000;
          }
          if (index2 == 13)
          {
            str3 = "30K".to_owned();
            num1 = 30000;
          }
          if (index2 == 14)
          {
            str3 = "50K".to_owned();
            num1 = 50000;
          }
          if (index2 == 0)
            color = Color.Red;
          if (index2 == 1)
            color = Color.Gray;
          if (index2 >= 2 & index2 <= 5)
            color = Color.White;
          if (index2 >= 6)
            color = Color.LightBlue;
          num9: i32;
          num10: i32;
          if (index2 <= 4)
          {
            num9 = num8 + index2 * 65;
            num10 = y1_1;
          }
          else if (index2 <= 9)
          {
            num9 = num8 + (index2 - 5) * 65;
            num10 = y1_1 + 42;
          }
          else if (index2 <= 14)
          {
            num9 = num8 + (index2 - 10) * 65;
            num10 = y1_1 + 84;
          }
          bool flag2 = false;
          let mut num11: i32 =  self.game.Data.RegimeObj[self.game.Data.Turn].Trafic2[0].Value[self.game.SelectX, self.game.SelectY];
          if (num11 != -1)
            num11 = Math.Abs(num11);
          if (num11 == num1)
            flag2 = true;
          if (flag2)
          {
            color = Color.FromArgb(150,  color.R,  color.G,  color.B);
            DrawMod.DrawBlock( graphics1, num9, num10, 56, 35,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredMarcCenter( graphics1, str3, self.game.MarcFont4, num9 + 19 + 10, num10 + 11, Color.White);
          }
          else
          {
            if (index2 == 1)
              color = Color.LightGray;
            tDescript7: String = "Click to set " + str3 + " Custom Pull Points on this Hex.";
            if (num1 == -1)
              tDescript7 = "Click to not allow any Asset,City or Unit Pull Points on this Hex.";
            if (num1 == 0)
              tDescript7 = "Click to remove Custom Pull Posettings: i32 from Hex.";
            int[] pullId = self.pullId;
            let mut index6: i32 =  index2;
            tsubpart8 =  new TextButtonPartClass(str3, 60, tDescript7,  self.OwnBitmap, num9, num10, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true, toverrulered: ( color.R), toverrulegreen: ( color.G), toverruleblue: ( color.B));
            let mut num12: i32 =  self.AddSubPart( tsubpart8, num9, num10, 60, 40, 1);
            pullId[index6] = num12;
            self.pullData[index2] = num1;
          }
          index2 += 1;
        }
        while (index2 <= 14);
        if (self.game.Data.RegimeObj[self.game.Data.Turn].Trafic2[0].Value[self.game.SelectX, self.game.SelectY] != 0)
        {
          tDescript8: String = "If Custom Pull Points are set to Addition Mode they'll be added to any Automatic Pull Points. If they are set to Overrule Mode they'll be the only Pull Points place on the Hex and any Automatic Pull Points will be ignored.";
          let mut num13: i32 =  self.game.Data.RegimeObj[self.game.Data.Turn].Trafic2[0].Value[self.game.SelectX, self.game.SelectY];
          if (num13 >= 0)
          {
            tsubpart8 =  ButtonPartClass::new(self.game.BUTTONFLAGGED, tDescript: tDescript8);
            self.overruleId = self.AddSubPart( tsubpart8, num8 + 12, y1_1 + 138, 32, 16, 1);
          }
          else
          {
            tsubpart8 =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED, tDescript: tDescript8);
            self.overruleId = self.AddSubPart( tsubpart8, num8 + 12, y1_1 + 138, 32, 16, 1);
          }
          tstring9: String = "Addition Mode";
          DrawMod.DrawTextColouredMarc( graphics1, tstring9, self.game.MarcFont4, num8 + 50, y1_1 + 136, Color.White);
          tDescript9: String = "If Custom Pull Points are set to Addition Mode they'll be added to any Automatic Pull Points. If they are set to Overrule Mode they'll be the only Pull Points place on the Hex and any Automatic Pull Points will be ignored.";
          if (num13 <= -1)
          {
            tsubpart8 =  ButtonPartClass::new(self.game.BUTTONFLAGGED, tDescript: tDescript9);
            self.overruleId2 = self.AddSubPart( tsubpart8, num8 + 12 + 160, y1_1 + 138, 32, 16, 1);
          }
          else
          {
            tsubpart8 =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED, tDescript: tDescript9);
            self.overruleId2 = self.AddSubPart( tsubpart8, num8 + 12 + 160, y1_1 + 138, 32, 16, 1);
          }
          str3 = "Overrule Mode";
          DrawMod.DrawTextColouredMarc( graphics1, str3, self.game.MarcFont4, num8 + 50 + 160, y1_1 + 136, Color.White);
        }
      }
      else
      {
        tstring10: String = "Start a new game";
        DrawMod.DrawTextColouredMarcCenter( graphics1, tstring10, self.game.MarcFont4, 210, 543, Color.Yellow);
        str3 = "for this feature";
        DrawMod.DrawTextColouredMarcCenter( graphics1, str3, self.game.MarcFont4, 210, 583, Color.Yellow);
      }
      let mut index7: i32 =  0;
      do
      {
        let mut x1_1: i32 =  0;
        if (index7 == 0)
        {
          x1_1 = x4 + 128 - 80;
          y1_1 = num7 - 80;
        }
        if (index7 == 1)
        {
          x1_1 = x4 + 256 - 16;
          y1_1 = num7 + 16;
        }
        if (index7 == 2)
        {
          x1_1 = x4 + 256 - 16;
          y1_1 = num7 + 16 + 96;
        }
        if (index7 == 3)
        {
          x1_1 = x4 + 128 - 80;
          y1_1 = num7 + 192 + 16;
        }
        if (index7 == 4)
        {
          x1_1 = x4 - 160 + 16;
          y1_1 = num7 + 16 + 96;
        }
        if (index7 == 5)
        {
          x1_1 = x4 - 160 + 16;
          y1_1 = num7 + 16;
        }
        if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].RoadType[index7] > -1 & x1_1 > 0)
        {
          index2 = 0;
          do
          {
            if (index2 == 0)
              str3 = "None".to_owned();
            if (index2 == 1)
              str3 = "20%";
            if (index2 == 2)
              str3 = "40%";
            if (index2 == 3)
              str3 = "60%";
            if (index2 == 4)
              str3 = "80%";
            if (index2 == 5)
              str3 = "90%";
            if (index2 == 6)
              str3 = "95%";
            if (index2 == 7)
              str3 = "All".to_owned();
            if (index2 == 0)
              color = Color.White;
            if (index2 == 1)
              color = Color.FromArgb( byte.MaxValue, 125,  byte.MaxValue, 125);
            if (index2 == 2)
              color = Color.FromArgb( byte.MaxValue, 0,  byte.MaxValue, 0);
            if (index2 == 3)
              color = Color.FromArgb( byte.MaxValue, 64, 152, 0);
            if (index2 == 4)
              color = Color.FromArgb( byte.MaxValue, 152, 152, 0);
            if (index2 == 5)
              color = Color.FromArgb( byte.MaxValue, 192, 128, 0);
            if (index2 == 6)
              color = Color.FromArgb( byte.MaxValue, 192, 64, 0);
            if (index2 == 7)
              color = Color.FromArgb( byte.MaxValue,  byte.MaxValue, 0, 100);
            num14: i32;
            num15: i32;
            if (index2 <= 3)
            {
              num14 = x1_1 + index2 * 40;
              num15 = y1_1;
            }
            else
            {
              num14 = x1_1 + (index2 - 4) * 40;
              num15 = y1_1 + 40;
            }
            bool flag3 = false;
            if (lisTraffic.data[index7] == index2)
              flag3 = true;
            if (flag3)
            {
              color = Color.FromArgb(100,  color.R,  color.G,  color.B);
              if (index2 == 0)
                color = Color.Gray;
              DrawMod.DrawBlock( graphics1, num14, num15, 36, 35,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredMarcCenter( graphics1, str3, self.game.MarcFont4, num14 + 19, num15 + 10, Color.White);
            }
            else
            {
              trafficId: Vec<i32> = self.trafficId;
              let mut index8: i32 =  index7;
              let mut index9: i32 =  index2;
              tsubpart8 =  new TextButtonPartClass(str3, 40, "Click to use Traffic Sign to stop " + str3 + " of logistical points leaving the Hex in this direction.",  self.OwnBitmap, num14, num15, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true, toverrulered: ( color.R), toverrulegreen: ( color.G), toverruleblue: ( color.B));
              let mut num16: i32 =  self.AddSubPart( tsubpart8, num14, num15, 40, 40, 1);
              trafficId[index8, index9] = num16;
            }
            index2 += 1;
          }
          while (index2 <= 7);
        }
        else
        {
          let mut num17: i32 =  x1_1 + index2 * 40;
          DrawMod.DrawBlock( graphics1, x1_1, y1_1, 160, 80, 0, 0, 0, 80);
          str3 = "No Road";
          DrawMod.DrawTextColouredMarcCenter( graphics1, str3, self.game.MarcFont4, x1_1 + 80, y1_1 + 30, Color.LightGray);
        }
        index7 += 1;
      }
      while (index7 <= 5);
      if (self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 505, 0, 0))].Width < 7)
      {
        tstring11: String = "Start a new game to re-enable this logs functionality. ";
        DrawMod.DrawTextColouredMarcCenter( graphics1, tstring11, self.game.MarcFont4, 770, 90, Color.LightGray);
      }
      else
      {
        let mut num18: i32 =  620;
        let mut num19: i32 =  90;
        tsubpart8 =  new TextButtonPartClass("Start Turn Log", 120, "Shows the Logistical Points generated on this Hex during start of turn.",  self.OwnBitmap, num18, num19, tred: (self.game.EditObj.layerLisPreview_LogMode == 0), theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.log1id = self.AddSubPart( tsubpart8, num18, num19, 120, 40, 1);
        let mut num20: i32 =  num18 + 130;
        if (self.game.EditObj.layerLisPreview)
        {
          tsubpart8 =  new TextButtonPartClass("Preview Log", 120, "Shows the Logistical Points that will be generated start of next turn.",  self.OwnBitmap, num20, num19, tred: (self.game.EditObj.layerLisPreview_LogMode == 1), theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.log2id = self.AddSubPart( tsubpart8, num20, num19, 120, 40, 1);
        }
        else
        {
          tsubpart8 =  new TextButtonPartClass("Preview Log", 120, "You have to activate the Preview Pts layer to inspect the Preview Logistics Log.",  self.OwnBitmap, num20, num19, true, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.log2Bid = self.AddSubPart( tsubpart8, num20, num19, 120, 40, 0);
        }
        let mut num21: i32 =  num20 + 130;
        if (self.game.EditObj.layerLisPreview & self.game.EditObj.layerLisOnlyAssetId > 0 & !self.game.EditObj.layerLisOnlyAssetId_isSupplyBase)
        {
          let mut idValue: i32 =   Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(9, self.game.EditObj.layerLisOnlyAssetId, 1)));
          let mut num22: i32 =   Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(9, self.game.EditObj.layerLisOnlyAssetId, 3)));
          let mut num23: i32 =   Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(9, self.game.EditObj.layerLisOnlyAssetId, 4)));
          str4: String = self.game.Data.StringListObj[stringListById2].GetData(0, idValue, 12);
          let mut nr: i32 =   Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, idValue, 2)));
          if (nr > 0)
            str4 = str4 + " " + self.game.HandyFunctionsObj.GetRomanNumerical(nr);
          tsubpart8 =  new TextButtonPartClass("Preview " + (str4 + " (" + num22.ToString() + "," + num23.ToString() + ")") + " Log", 260, "Shows the Logistical Points that will be generated by this specific Asset at the start of next turn.",  self.OwnBitmap, num21, num19, tred: (self.game.EditObj.layerLisPreview_LogMode == 2), theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.log3id = self.AddSubPart( tsubpart8, num21, num19, 260, 40, 1);
        }
        else if (self.game.EditObj.layerLisOnlyAssetId > 0 & self.game.EditObj.layerLisOnlyAssetId_isSupplyBase)
        {
          tsubpart8 =  new TextButtonPartClass("Preview Asset Log", 260, "Supply Bases do not generate Logistical Points logs.",  self.OwnBitmap, num21, num19, true, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.log3Bid = self.AddSubPart( tsubpart8, num21, num19, 260, 40, 0);
        }
        else
        {
          tsubpart8 =  new TextButtonPartClass("Preview Asset Log", 260, "You have to activate the Preview Pts layer and select a specific Asset to inspect the Preview Logistics Log.",  self.OwnBitmap, num21, num19, true, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.log3Bid = self.AddSubPart( tsubpart8, num21, num19, 260, 40, 0);
        }
        let mut num24: i32 =  620;
        let mut num25: i32 =  140;
        let mut num26: i32 =  0;
        if (self.game.EditObj.layerLisPreview_LogMode <= 1)
        {
          tsubpart8 =  new TextButtonPartClass("All Logs", 200, "Shows all the Logs for this Hex.",  self.OwnBitmap, num24, num25, tred: (self.game.EditObj.layerLis_LogType == 0), theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.logType1id = self.AddSubPart( tsubpart8, num24, num25, 200, 40, 1);
          let mut num27: i32 =  num24 + 220;
          tsubpart8 =  new TextButtonPartClass("Logs per Source Hex", 200, "Shows the Logs but selectable per Logistical Points Source Hex.",  self.OwnBitmap, num27, num25, tred: (self.game.EditObj.layerLis_LogType == 1), theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.logType2id = self.AddSubPart( tsubpart8, num27, num25, 200, 40, 1);
          num26 = 2;
          num25 += 50;
          num24 = 620;
        }
        else
          self.game.EditObj.layerLis_LogType = 0;
        let mut index10: i32 =  0;
        if (self.game.EditObj.layerLisPreview_LogMode < 1)
          index10 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 505, 0, 0));
        else if (self.game.EditObj.layerLisPreview_LogMode == 1)
          index10 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 511, 0, 0));
        else if (self.game.EditObj.layerLisPreview_LogMode == 2)
          index10 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 512, 0, 0));
        if (index10 <= -1)
          return;
        let mut num28: i32 =  520;
        self.LogSourceList = SimpleList::new();
        if (self.game.EditObj.layerLis_LogType == 1)
        {
          let mut length: i32 =  self.game.Data.StringListObj[index10].Length;
          for (let mut index11: i32 =  0; index11 <= length; index11 += 1)
          {
            if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[index10].Data[index11, 0])) == self.game.SelectX &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[index10].Data[index11, 1])) == self.game.SelectY)
            {
              let mut tdata1: i32 =   Math.Round(Conversion.Val(self.game.Data.StringListObj[index10].Data[index11, 5]));
              if (tdata1 > -1)
              {
                let mut tdata2: i32 =   Math.Round(Conversion.Val(self.game.Data.StringListObj[index10].Data[index11, 6]));
                let mut tdata3: i32 =   Math.Round(Conversion.Val(self.game.Data.StringListObj[index10].Data[index11, 7]));
                let mut tid: i32 =  tdata3 * 1000000 + tdata1 * 1000 + tdata2;
                if (tdata3 >= 1 & tdata3 <= 2)
                  self.LogSourceList.AddWeight(tid, 1, tdata1, tdata2, tdata3);
              }
            }
          }
          self.OptionsListObj = ListClass::new();
          let mut num29: i32 =  -1;
          let mut tlistselect: i32 =  -1;
          let mut counter: i32 =  self.LogSourceList.Counter;
          for (let mut index12: i32 =  0; index12 <= counter; index12 += 1)
          {
            num29 += 1;
            self.OptionsListObj.add(self.game.Data.StringListObj[stringListById3].GetData(0, self.LogSourceList.Data3[index12], 1) + " (" + self.LogSourceList.Data1[index12].ToString() + "," + self.LogSourceList.Data2[index12].ToString() + ")", self.LogSourceList.Id[index12]);
            if (self.LogSourceList.Data1[index12] == self.logSourceX & self.LogSourceList.Data2[index12] == self.logSourceY & self.LogSourceList.Data3[index12] == self.logSourceType)
              tlistselect = num29;
          }
          if (tlistselect == -1)
          {
            tlistselect = 0;
            if (self.LogSourceList.Counter >= 0)
            {
              self.logSourceX = self.LogSourceList.Data1[0];
              self.logSourceY = self.LogSourceList.Data2[0];
              self.logSourceType = self.LogSourceList.Data3[0];
            }
          }
          if (self.OptionsListId > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect);
            self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
          }
          else
          {
            tsubpart8 =  new ListSubPartClass(self.OptionsListObj, 22 - num26, 120, tlistselect, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num24, bby: num25, tMarcStyle: true, overruleFont: ( self.game.MarcFont7), overruleItemSize: 20);
            self.OptionsListId = self.AddSubPart( tsubpart8, num24, num25, 120, 20 * (21 - num26), 0);
          }
          num28 -= 130;
          num24 += 130;
          if (self.OptionsList2Id > 0)
          {
            self.RemoveSubPart(self.OptionsList2Id);
            self.OptionsList2Id = 0;
          }
        }
        else if (self.OptionsListId > 0)
        {
          self.RemoveSubPart(self.OptionsListId);
          self.OptionsListId = 0;
        }
        self.OptionsList2Obj = ListClass::new();
        let mut length1: i32 =  self.game.Data.StringListObj[index10].Length;
        for (let mut tdata: i32 =  0; tdata <= length1; tdata += 1)
        {
          if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[index10].Data[tdata, 0])) == self.game.SelectX &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[index10].Data[tdata, 1])) == self.game.SelectY)
          {
            tname: String = self.game.Data.StringListObj[index10].Data[tdata, 4];
            if (self.game.EditObj.layerLis_LogType < 1)
            {
              self.OptionsList2Obj.add(tname, tdata);
            }
            else
            {
              let mut num30: i32 =   Math.Round(Conversion.Val(self.game.Data.StringListObj[index10].Data[tdata, 5]));
              let mut num31: i32 =   Math.Round(Conversion.Val(self.game.Data.StringListObj[index10].Data[tdata, 6]));
              let mut num32: i32 =   Math.Round(Conversion.Val(self.game.Data.StringListObj[index10].Data[tdata, 7]));
              if (num30 == self.logSourceX & num31 == self.logSourceY & (num32 == self.logSourceType | num32 == 0))
                self.OptionsList2Obj.add(tname, tdata);
            }
          }
        }
        if (self.OptionsList2Id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList2Id)].Refresh(self.OptionsList2Obj, -1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList2Id)] = true;
        }
        else
        {
          tsubpart8 =  new ListSubPartClass(self.OptionsList2Obj, 22 - num26, num28, -1, self.game, tHeaderCenter: false, tHighlight: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num24, bby: num25, tMarcStyle: true, overruleFont: ( self.game.MarcFont7), overruleItemSize: 20);
          self.OptionsList2Id = self.AddSubPart( tsubpart8, num24, num25, num28, 20 * (21 - num26), 0);
        }
      }
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27)
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

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  self.SubPartCounter;
label_197:
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num1: i32 =  self.SubPartID[index1];
            if (num1 == self.logType1id)
            {
              self.game.EditObj.layerLis_LogType = 0;
              if (self.OptionsList2Id > 0)
              {
                self.RemoveSubPart(self.OptionsList2Id);
                self.OptionsList2Id = 0;
              }
              self.Setup();
              self.View();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.logType2id)
            {
              self.game.EditObj.layerLis_LogType = 1;
              if (self.OptionsList2Id > 0)
              {
                self.RemoveSubPart(self.OptionsList2Id);
                self.OptionsList2Id = 0;
              }
              self.Setup();
              self.View();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.log1id)
            {
              self.game.EditObj.layerLisPreview_LogMode = 0;
              if (self.OptionsList2Id > 0)
              {
                self.RemoveSubPart(self.OptionsList2Id);
                self.OptionsList2Id = 0;
              }
              if (self.OptionsListId > 0)
              {
                self.RemoveSubPart(self.OptionsListId);
                self.OptionsListId = 0;
              }
              self.Setup();
              self.View();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.log2id)
            {
              self.game.EditObj.layerLisPreview_LogMode = 1;
              if (self.OptionsList2Id > 0)
              {
                self.RemoveSubPart(self.OptionsList2Id);
                self.OptionsList2Id = 0;
              }
              if (self.OptionsListId > 0)
              {
                self.RemoveSubPart(self.OptionsListId);
                self.OptionsListId = 0;
              }
              self.Setup();
              self.View();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.log3id)
            {
              self.game.EditObj.layerLisPreview_LogMode = 2;
              if (self.OptionsList2Id > 0)
              {
                self.RemoveSubPart(self.OptionsList2Id);
                self.OptionsList2Id = 0;
              }
              if (self.OptionsListId > 0)
              {
                self.RemoveSubPart(self.OptionsListId);
                self.OptionsListId = 0;
              }
              self.Setup();
              self.View();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.okid)
            {
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsList2Id)
            {
              self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsListId)
            {
              let mut tid: i32 =  self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              if (tid > 0)
              {
                let mut nr: i32 =  self.LogSourceList.FindNr(tid);
                if (nr > -1)
                {
                  self.logSourceX = self.LogSourceList.Data1[nr];
                  self.logSourceY = self.LogSourceList.Data2[nr];
                  self.logSourceType = self.LogSourceList.Data3[nr];
                  if (self.OptionsList2Id > 0)
                  {
                    self.RemoveSubPart(self.OptionsList2Id);
                    self.OptionsList2Id = 0;
                  }
                }
              }
              self.Setup();
              self.View();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.clearAll2id)
            {
              let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
              for (let mut index2: i32 =  0; index2 <= mapWidth1; index2 += 1)
              {
                let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
                {
                  let mut num2: i32 =  0;
                  do
                  {
                    self.game.Data.RegimeObj[self.game.Data.Turn].Trafic2[0].Value[index2, index3] = 0;
                    num2 += 1;
                  }
                  while (num2 <= 8);
                }
              }
              if (self.game.EditObj.layerLisPreview)
              {
                let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
                for (let mut index4: i32 =  0; index4 <= mapWidth2; index4 += 1)
                {
                  let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                  for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
                  {
                    let mut index6: i32 =  0;
                    do
                    {
                      self.game.Data.MapObj[0].HexObj[index4, index5].tempPreviewLIS[index6] = 0;
                      self.game.Data.MapObj[0].HexObj[index4, index5].tempPreviewAssetLIS[index6] = 0;
                      index6 += 1;
                    }
                    while (index6 <= 8);
                  }
                }
                self.game.ProcessingObj.LIS_SetNetwork(false, true);
                if (self.game.EditObj.layerLisOnlyAssetId > 0)
                  self.game.ProcessingObj.LIS_SetNetwork(false, true, self.game.EditObj.layerLisOnlyAssetId);
              }
              self.Setup();
              self.View();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.flag1id || num1 == self.flag2id || num1 == self.flag3id)
            {
              let mut stringListById: i32 =  self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 210, 0, 0));
              if (self.flag1id == self.SubPartID[index1])
              {
                setValue: i32;
                if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "pullAssetsOff", 2))) == 0)
                {
                  setValue = 1;
                  self.game.EditObj.usePullAssets = false;
                }
                else
                {
                  setValue = 0;
                  self.game.EditObj.usePullAssets = true;
                }
                self.game.Data.StringListObj[stringListById].SetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "pullAssetsOff", 2, setValue, true);
              }
              if (self.flag2id == self.SubPartID[index1])
              {
                setValue: i32;
                if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "pullUnitsOff", 2))) == 0)
                {
                  setValue = 1;
                  self.game.EditObj.usePullUnits = false;
                }
                else
                {
                  setValue = 0;
                  self.game.EditObj.usePullUnits = true;
                }
                self.game.Data.StringListObj[stringListById].SetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "pullUnitsOff", 2, setValue, true);
              }
              if (self.flag3id == self.SubPartID[index1])
              {
                setValue: i32;
                if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "pullCitiesOff", 2))) == 0)
                {
                  setValue = 1;
                  self.game.EditObj.usePullCities = false;
                }
                else
                {
                  setValue = 0;
                  self.game.EditObj.usePullCities = true;
                }
                self.game.Data.StringListObj[stringListById].SetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "pullCitiesOff", 2, setValue, true);
              }
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              if (self.game.EditObj.layerLisPreview)
              {
                let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
                for (let mut index7: i32 =  0; index7 <= mapWidth; index7 += 1)
                {
                  let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                  for (let mut index8: i32 =  0; index8 <= mapHeight; index8 += 1)
                  {
                    let mut index9: i32 =  0;
                    do
                    {
                      self.game.Data.MapObj[0].HexObj[index7, index8].tempPreviewLIS[index9] = 0;
                      self.game.Data.MapObj[0].HexObj[index7, index8].tempPreviewAssetLIS[index9] = 0;
                      index9 += 1;
                    }
                    while (index9 <= 8);
                  }
                }
                self.game.ProcessingObj.LIS_SetNetwork(false, true);
                if (self.game.EditObj.layerLisOnlyAssetId > 0)
                  self.game.ProcessingObj.LIS_SetNetwork(false, true, self.game.EditObj.layerLisOnlyAssetId);
              }
              self.Setup();
              self.View();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.clearAllid)
            {
              let mut mapWidth3: i32 =  self.game.Data.MapObj[0].MapWidth;
              for (let mut index10: i32 =  0; index10 <= mapWidth3; index10 += 1)
              {
                let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                for (let mut index11: i32 =  0; index11 <= mapHeight; index11 += 1)
                {
                  let mut num3: i32 =  0;
                  do
                  {
                    self.game.Data.RegimeObj[self.game.Data.Turn].Trafic[0].Value[index10, index11] = 0;
                    num3 += 1;
                  }
                  while (num3 <= 8);
                }
              }
              if (self.game.EditObj.layerLisPreview)
              {
                let mut mapWidth4: i32 =  self.game.Data.MapObj[0].MapWidth;
                for (let mut index12: i32 =  0; index12 <= mapWidth4; index12 += 1)
                {
                  let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                  for (let mut index13: i32 =  0; index13 <= mapHeight; index13 += 1)
                  {
                    let mut index14: i32 =  0;
                    do
                    {
                      self.game.Data.MapObj[0].HexObj[index12, index13].tempPreviewLIS[index14] = 0;
                      self.game.Data.MapObj[0].HexObj[index12, index13].tempPreviewAssetLIS[index14] = 0;
                      index14 += 1;
                    }
                    while (index14 <= 8);
                  }
                }
                self.game.ProcessingObj.LIS_SetNetwork(false, true);
                if (self.game.EditObj.layerLisOnlyAssetId > 0)
                  self.game.ProcessingObj.LIS_SetNetwork(false, true, self.game.EditObj.layerLisOnlyAssetId);
              }
              self.Setup();
              self.View();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.flagTruckId || num1 == self.flagRailId || num1 == self.flagPullId)
            {
              NeighboursExtra lisTraffic = self.game.HandyFunctionsObj.GetLisTraffic(self.game.SelectX, self.game.SelectY);
              if (self.flagTruckId == self.SubPartID[index1])
                lisTraffic.truck = !lisTraffic.truck;
              else if (self.flagRailId == self.SubPartID[index1])
                lisTraffic.rail = !lisTraffic.rail;
              else if (self.flagPullId == self.SubPartID[index1])
                lisTraffic.pull = !lisTraffic.pull;
              self.game.HandyFunctionsObj.SetLisTraffic(self.game.SelectX, self.game.SelectY, lisTraffic);
              if (self.game.EditObj.layerLisPreview)
              {
                let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
                for (let mut index15: i32 =  0; index15 <= mapWidth; index15 += 1)
                {
                  let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                  for (let mut index16: i32 =  0; index16 <= mapHeight; index16 += 1)
                  {
                    let mut index17: i32 =  0;
                    do
                    {
                      self.game.Data.MapObj[0].HexObj[index15, index16].tempPreviewLIS[index17] = 0;
                      self.game.Data.MapObj[0].HexObj[index15, index16].tempPreviewAssetLIS[index17] = 0;
                      index17 += 1;
                    }
                    while (index17 <= 8);
                  }
                }
                self.game.ProcessingObj.LIS_SetNetwork(false, true);
                if (self.game.EditObj.layerLisOnlyAssetId > 0)
                  self.game.ProcessingObj.LIS_SetNetwork(false, true, self.game.EditObj.layerLisOnlyAssetId);
              }
              self.Setup();
              self.View();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.overruleId || num1 == self.overruleId2)
            {
              let mut num4: i32 =  self.game.Data.RegimeObj[self.game.Data.Turn].Trafic2[0].Value[self.game.SelectX, self.game.SelectY];
              switch (num4)
              {
                case -1:
                case 0:
                  self.Setup();
                  self.View();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                default:
                  self.game.Data.RegimeObj[self.game.Data.Turn].Trafic2[0].Value[self.game.SelectX, self.game.SelectY] = self.overruleId != self.SubPartID[index1] ? -Math.Abs(num4) : Math.Abs(num4);
                  if (self.game.EditObj.layerLisPreview)
                  {
                    let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
                    for (let mut index18: i32 =  0; index18 <= mapWidth; index18 += 1)
                    {
                      let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                      for (let mut index19: i32 =  0; index19 <= mapHeight; index19 += 1)
                      {
                        let mut index20: i32 =  0;
                        do
                        {
                          self.game.Data.MapObj[0].HexObj[index18, index19].tempPreviewLIS[index20] = 0;
                          self.game.Data.MapObj[0].HexObj[index18, index19].tempPreviewAssetLIS[index20] = 0;
                          index20 += 1;
                        }
                        while (index20 <= 8);
                      }
                    }
                    self.game.ProcessingObj.LIS_SetNetwork(false, true);
                    if (self.game.EditObj.layerLisOnlyAssetId > 0)
                    {
                      self.game.ProcessingObj.LIS_SetNetwork(false, true, self.game.EditObj.layerLisOnlyAssetId);
                      goto case -1;
                    }
                    else
                      goto case -1;
                  }
                  else
                    goto case -1;
              }
            }
            else
            {
              let mut index21: i32 =  0;
label_143:
              let mut index22: i32 =  0;
              while (self.trafficId[index21, index22] != self.SubPartID[index1])
              {
                index22 += 1;
                if (index22 > 7)
                {
                  index21 += 1;
                  if (index21 > 5)
                  {
                    let mut index23: i32 =  0;
label_160:
                    let mut index24: i32 =  0;
                    while (self.trafficId[index23, index24] != self.SubPartID[index1])
                    {
                      index24 += 1;
                      if (index24 > 7)
                      {
                        index23 += 1;
                        if (index23 > 5)
                        {
                          let mut index25: i32 =  0;
                          while (self.SubPartID[index1] != self.pullId[index25])
                          {
                            index25 += 1;
                            if (index25 > 15)
                              goto label_197;
                          }
                          if (self.pullData[index25] == -1)
                            self.game.Data.RegimeObj[self.game.Data.Turn].Trafic2[0].Value[self.game.SelectX, self.game.SelectY] = self.pullData[index25];
                          else if (self.game.Data.RegimeObj[self.game.Data.Turn].Trafic2[0].Value[self.game.SelectX, self.game.SelectY] <= -1)
                            self.game.Data.RegimeObj[self.game.Data.Turn].Trafic2[0].Value[self.game.SelectX, self.game.SelectY] = -self.pullData[index25];
                          else
                            self.game.Data.RegimeObj[self.game.Data.Turn].Trafic2[0].Value[self.game.SelectX, self.game.SelectY] = self.pullData[index25];
                          if (self.game.EditObj.layerLisPreview)
                          {
                            let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
                            for (let mut index26: i32 =  0; index26 <= mapWidth; index26 += 1)
                            {
                              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                              for (let mut index27: i32 =  0; index27 <= mapHeight; index27 += 1)
                              {
                                let mut index28: i32 =  0;
                                do
                                {
                                  self.game.Data.MapObj[0].HexObj[index26, index27].tempPreviewLIS[index28] = 0;
                                  self.game.Data.MapObj[0].HexObj[index26, index27].tempPreviewAssetLIS[index28] = 0;
                                  index28 += 1;
                                }
                                while (index28 <= 8);
                              }
                            }
                            self.game.ProcessingObj.LIS_SetNetwork(false, true);
                            if (self.game.EditObj.layerLisOnlyAssetId > 0)
                              self.game.ProcessingObj.LIS_SetNetwork(false, true, self.game.EditObj.layerLisOnlyAssetId);
                          }
                          self.Setup();
                          self.View();
                          windowReturnClass.SetFlag(true);
                          return windowReturnClass;
                        }
                        goto label_160;
                      }
                    }
                    NeighboursExtra lisTraffic = self.game.HandyFunctionsObj.GetLisTraffic(self.game.SelectX, self.game.SelectY);
                    lisTraffic.data[index23] = index24;
                    self.game.HandyFunctionsObj.SetLisTraffic(self.game.SelectX, self.game.SelectY, lisTraffic);
                    if (self.game.EditObj.layerLisPreview)
                    {
                      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
                      for (let mut index29: i32 =  0; index29 <= mapWidth; index29 += 1)
                      {
                        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                        for (let mut index30: i32 =  0; index30 <= mapHeight; index30 += 1)
                        {
                          let mut index31: i32 =  0;
                          do
                          {
                            self.game.Data.MapObj[0].HexObj[index29, index30].tempPreviewLIS[index31] = 0;
                            self.game.Data.MapObj[0].HexObj[index29, index30].tempPreviewAssetLIS[index31] = 0;
                            index31 += 1;
                          }
                          while (index31 <= 8);
                        }
                      }
                      self.game.ProcessingObj.LIS_SetNetwork(false, true);
                      if (self.game.EditObj.layerLisOnlyAssetId > 0)
                        self.game.ProcessingObj.LIS_SetNetwork(false, true, self.game.EditObj.layerLisOnlyAssetId);
                    }
                    self.Setup();
                    self.View();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  goto label_143;
                }
              }
              NeighboursExtra lisTraffic1 = self.game.HandyFunctionsObj.GetLisTraffic(self.game.SelectX, self.game.SelectY);
              lisTraffic1.data[index21] = index22;
              self.game.HandyFunctionsObj.SetLisTraffic(self.game.SelectX, self.game.SelectY, lisTraffic1);
              if (self.game.EditObj.layerLisPreview)
              {
                let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
                for (let mut index32: i32 =  0; index32 <= mapWidth; index32 += 1)
                {
                  let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                  for (let mut index33: i32 =  0; index33 <= mapHeight; index33 += 1)
                  {
                    let mut index34: i32 =  0;
                    do
                    {
                      self.game.Data.MapObj[0].HexObj[index32, index33].tempPreviewLIS[index34] = 0;
                      self.game.Data.MapObj[0].HexObj[index32, index33].tempPreviewAssetLIS[index34] = 0;
                      index34 += 1;
                    }
                    while (index34 <= 8);
                  }
                }
                self.game.ProcessingObj.LIS_SetNetwork(false, true);
                if (self.game.EditObj.layerLisOnlyAssetId > 0)
                  self.game.ProcessingObj.LIS_SetNetwork(false, true, self.game.EditObj.layerLisOnlyAssetId);
              }
              self.Setup();
              self.View();
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
