// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.HistoryWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Threading;

namespace WindowsApplication1
{
  pub class HistoryWindowClass2 : WindowClass
  {
     Info1Id: i32;
     info3id: i32;
     mapid: i32;
     info4id: i32;
     info5id: i32;
     info6id: i32;
     ViewAntiCapId: i32;
     ViewAntiCapTextId: i32;
     ViewAntiCap2Id: i32;
     ViewAntiCapText2Id: i32;
     ViewAntiCap3Id: i32;
     ViewAntiCapText3Id: i32;
     ViewHistoryId: i32;
     ViewHistoryTextId: i32;
     ViewSupplyId: i32;
     ViewSupplyTextId: i32;
     Slider1Id: i32;
    pub StartStep: i32;
    pub EndStep: i32;
    pub TotStep: i32;
    pub Curstep: i32;
    pub RealStepNr: i32;
    pub DateTime showtime;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
    pub AutoPlay: bool;
     SpecialTextId: i32;
    pub detail1: i32;
    pub detail2: i32;
    pub HumanPlayer: i32;
     Turny: i32;
    pub detailnr: i32;
    pub lastregime: i32;
     w: i32;
     h: i32;
     lastendstep: i32;

    pub HistoryWindowClass2( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base( tGame, tGame.ScreenWidth, 222, BackSprite: tGame.MARCBOTBAR)
    {
      self.w = tGame.ScreenWidth;
      self.h = 222;
      self.BlockBlit = true;
      if (!self.game.EditObj.AIMoving)
      {
        self.game.EditObj.RealRound = self.game.Data.Round;
        self.game.EditObj.RealTurn = self.game.Data.Turn;
      }
      self.StartStep = -1;
      self.EndStep = 0;
      self.lastendstep = 0;
      self.TotStep = -1;
      self.Curstep = 0;
      self.HumanPlayer = -1;
      self.Turny = self.game.EditObj.RealTurn;
      if (self.game.EditObj.TempAIWatch)
      {
        self.HumanPlayer = self.game.EditObj.HumanPlayer;
        self.Turny = self.game.EditObj.HumanPlayer;
      }
      self.detailnr = self.game.EditObj.MapSelected;
      self.game.EditObj.MiniMap = new Bitmap(1, 1);
      if (self.game.EditObj.TempAIWatch)
      {
        self.AutoPlay = true;
        self.game.EditObj.AIMoving = true;
      }
      self.game.EditObj.TempAIWatch = false;
      self.game.EditObj.OrderType = 26;
      self.showtime = DateAndTime.Now;
      self.game.EditObj.HisInfoString = "";
      self.game.EditObj.HisLossCounter = -1;
      if (self.game.Data.RegimeObj[self.Turny].HistoryStepCounter > -1)
      {
        self.StartStep = 1;
        self.EndStep = self.game.HandyFunctionsObj.GetRegimeHistoryTotSteps(self.Turny);
      }
      self.StartSit();
      if (self.game.EditObj.LastHistoryStep > 0)
      {
        self.Forward(self.game.EditObj.LastHistoryStep);
        self.Curstep = self.game.EditObj.LastHistoryStep;
        self.game.EditObj.LastHistoryStep = -1;
      }
      self.dostuff();
    }

    pub fn StartSit()
    {
      self.game.EditObj.HisForce = new MapMatrix2[self.game.Data.MapCounter + 1];
      self.game.EditObj.HisSFType = new MapMatrix2[self.game.Data.MapCounter + 1];
      self.game.EditObj.HisOwner = new MapMatrix2[self.game.Data.MapCounter + 1];
      self.game.EditObj.HisHis = new MapMatrix2[self.game.Data.MapCounter + 1];
      self.game.EditObj.HisDepth = new MapMatrix2[self.game.Data.MapCounter + 1];
      let mut mapCounter: i32 =  self.game.Data.MapCounter;
      for (let mut index: i32 =  0; index <= mapCounter; index += 1)
      {
        self.game.EditObj.HisForce[index] = self.game.Data.RegimeObj[self.Turny].HistoryForce[index].Clone();
        self.game.EditObj.HisSFType[index] = self.game.Data.RegimeObj[self.Turny].HistorySFType[index].Clone();
        self.game.EditObj.HisOwner[index] = self.game.Data.RegimeObj[self.Turny].HistoryOwner[index].Clone();
        self.game.EditObj.HisHis[index] = self.game.Data.RegimeObj[self.Turny].HistoryHis[index].Clone();
        self.game.EditObj.HisDepth[index] = self.game.Data.RegimeObj[self.Turny].HistoryDepth[index].Clone();
      }
      if (self.game.Data.RegimeObj[self.Turny].HistoryStepCounter > -1)
        self.RealStepNr = self.game.Data.RegimeObj[self.Turny].HistoryStep[0].StepNr - 1;
      self.game.EditObj.HisHotX = -1;
      self.game.EditObj.HisHotY = -1;
      self.game.EditObj.HisHotMap = -1;
      self.game.EditObj.HisLossCounter = -1;
      self.game.EditObj.HisLossAttacker = new int[1];
      self.game.EditObj.HisLossDEAD = new int[1];
      self.game.EditObj.HisLossOK = new int[1];
      self.game.EditObj.HisLossSFType = new int[1];
      self.game.EditObj.HisRegimeWon = -1;
      self.game.EditObj.HisAttackType = -1;
      self.game.EditObj.TempCoordList = CoordList::new();
    }

    pub fn Forward(steps: i32)
    {
      let mut num1: i32 =  -1;
      let mut num2: i32 =  0;
      self.game.EditObj.HisHotX = -1;
      self.game.EditObj.HisHotY = -1;
      self.game.EditObj.HisHotMap = -1;
      self.game.EditObj.HisLossAttacker = new int[1];
      self.game.EditObj.HisLossDEAD = new int[1];
      self.game.EditObj.HisLossOK = new int[1];
      self.game.EditObj.HisLossSFType = new int[1];
      self.game.EditObj.HisLossCounter = -1;
      self.game.EditObj.HisRegimeWon = -1;
      self.game.EditObj.HisLossDefReg = -1;
      self.game.EditObj.HisLossAttReg = -1;
      self.game.EditObj.HisInfoString = "";
      self.game.EditObj.HisAttackType = -1;
      let mut num3: i32 =  0;
      let mut historyStepCounter: i32 =  self.game.Data.RegimeObj[self.Turny].HistoryStepCounter;
      for (let mut index1: i32 =  0; index1 <= historyStepCounter; index1 += 1)
      {
        HistoryStepClass historyStepClass = self.game.Data.RegimeObj[self.Turny].HistoryStep[index1];
        if (historyStepClass.StepNr > self.RealStepNr)
        {
          if (historyStepClass.StepNr != num1)
          {
            num2 += 1;
            num1 = historyStepClass.StepNr;
            if (num2 > steps)
            {
              self.RealStepNr = historyStepClass.StepNr - 1;
              break;
            }
            self.game.EditObj.HisLossCounter = -1;
            self.game.EditObj.HisRegimeWon = -1;
            self.game.EditObj.HisLossDefReg = -1;
            self.game.EditObj.HisLossAttReg = -1;
            self.game.EditObj.HisInfoString = "";
            self.game.EditObj.HisAttackType = -1;
          }
          else if (num2 > steps)
          {
            self.RealStepNr = historyStepClass.StepNr - 1;
            break;
          }
          self.lastregime = historyStepClass.Regime;
          self.game.EditObj.HisForce[historyStepClass.Map].Value[historyStepClass.X, historyStepClass.Y] = historyStepClass.Force;
          self.game.EditObj.HisSFType[historyStepClass.Map].Value[historyStepClass.X, historyStepClass.Y] = historyStepClass.SFType;
          self.game.EditObj.HisOwner[historyStepClass.Map].Value[historyStepClass.X, historyStepClass.Y] = historyStepClass.Ownership;
          self.game.EditObj.HisHis[historyStepClass.Map].Value[historyStepClass.X, historyStepClass.Y] = historyStepClass.His;
          self.game.EditObj.HisDepth[historyStepClass.Map].Value[historyStepClass.X, historyStepClass.Y] = historyStepClass.Depth;
          self.game.EditObj.TempCoordList.AddCoord(historyStepClass.X, historyStepClass.Y, historyStepClass.Map);
          let mut tfacing: i32 =  1;
          do
          {
            Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(historyStepClass.X, historyStepClass.Y, historyStepClass.Map, tfacing);
            if (coordinate.onmap)
              self.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
            tfacing += 1;
          }
          while (tfacing <= 6);
          self.game.HandyFunctionsObj.SetcornerXY2(historyStepClass.X, historyStepClass.Y);
          if (self.game.EditObj.MapSelected != historyStepClass.Map)
          {
            self.game.EditObj.MapSelected = historyStepClass.Map;
            self.game.CornerX = 0;
            self.game.CornerY = 0;
            num3 = 1;
          }
          if (Strings.Len(historyStepClass.InfoString) > 0)
            self.game.EditObj.HisInfoString = historyStepClass.InfoString;
          if (historyStepClass.AttackOtherType > -1)
          {
            self.game.EditObj.HisHotX = historyStepClass.X;
            self.game.EditObj.HisHotY = historyStepClass.Y;
            self.game.EditObj.HisHotMap = historyStepClass.Map;
            self.game.EditObj.HisNeighbour = Neighbours::new();
            let mut index2: i32 =  0;
            do
            {
              self.game.EditObj.HisNeighbour.data[index2] = historyStepClass.AttackDirection[index2];
              index2 += 1;
            }
            while (index2 <= 5);
            self.game.EditObj.HisAttackType = historyStepClass.AttackOtherType;
          }
          else
          {
            self.game.EditObj.HisHotX = historyStepClass.X;
            self.game.EditObj.HisHotY = historyStepClass.Y;
            self.game.EditObj.HisHotMap = historyStepClass.Map;
          }
          if (historyStepClass.LossCounter > -1)
          {
            self.game.EditObj.HisLossAttacker = new int[historyStepClass.LossCounter + 1];
            self.game.EditObj.HisLossDEAD = new int[historyStepClass.LossCounter + 1];
            self.game.EditObj.HisLossOK = new int[historyStepClass.LossCounter + 1];
            self.game.EditObj.HisLossSFType = new int[historyStepClass.LossCounter + 1];
            self.game.EditObj.HisLossCounter = historyStepClass.LossCounter;
            let mut lossCounter: i32 =  historyStepClass.LossCounter;
            for (let mut index3: i32 =  0; index3 <= lossCounter; index3 += 1)
            {
              self.game.EditObj.HisLossAttacker[index3] = historyStepClass.LossAttacker[index3];
              self.game.EditObj.HisLossDEAD[index3] = historyStepClass.LossDEAD[index3];
              self.game.EditObj.HisLossOK[index3] = historyStepClass.LossOK[index3];
              self.game.EditObj.HisLossSFType[index3] = historyStepClass.LossSFType[index3];
            }
            self.game.EditObj.HisRegimeWon = historyStepClass.LossRegimeWin;
            self.game.EditObj.HisLossAttReg = historyStepClass.LossAttReg;
            self.game.EditObj.HisLossDefReg = historyStepClass.LossDefReg;
          }
          if (historyStepClass.LossCounter == -1 & historyStepClass.AttackOtherType > -1 && self.game.Data.RegimeObj[self.Turny].HistoryStepCounter >= index1 + 1 && historyStepClass.X == self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].X & historyStepClass.Y == self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].Y && self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossCounter > -1 && self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossCounter > -1)
          {
            self.game.EditObj.HisLossAttacker = new int[self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossCounter + 1];
            self.game.EditObj.HisLossDEAD = new int[self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossCounter + 1];
            self.game.EditObj.HisLossOK = new int[self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossCounter + 1];
            self.game.EditObj.HisLossSFType = new int[self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossCounter + 1];
            self.game.EditObj.HisLossCounter = self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossCounter;
            let mut lossCounter: i32 =  self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossCounter;
            for (let mut index4: i32 =  0; index4 <= lossCounter; index4 += 1)
            {
              self.game.EditObj.HisLossAttacker[index4] = self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossAttacker[index4];
              self.game.EditObj.HisLossDEAD[index4] = self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossDEAD[index4];
              self.game.EditObj.HisLossOK[index4] = self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossOK[index4];
              self.game.EditObj.HisLossSFType[index4] = self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossSFType[index4];
            }
            self.game.EditObj.HisRegimeWon = self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossRegimeWin;
            self.game.EditObj.HisLossAttReg = self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossAttReg;
            self.game.EditObj.HisLossDefReg = self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].LossDefReg;
            if (Strings.Len(historyStepClass.InfoString) == 0 && Strings.Len(self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].InfoString) > 0)
              self.game.EditObj.HisInfoString = self.game.Data.RegimeObj[self.Turny].HistoryStep[index1 + 1].InfoString;
          }
        }
      }
      let mut num4: i32 =   Math.Round( self.game.ScreenWidth / 53.0);
      let mut num5: i32 =   Math.Round( (self.game.ScreenHeight - 200) / 48.0);
      let mut num6: i32 =  0;
      if (self.game.EditObj.HisHotX > -1 & self.game.EditObj.HisHotY > -1)
      {
        if (self.game.Data.MapObj[0].MapLoop)
        {
          if (self.game.EditObj.HisHotX <= self.game.CornerX + 1)
          {
            let mut num7: i32 =  self.game.CornerX + self.game.Data.MapObj[0].MapWidth + num4 + 5;
            if (num7 > self.game.Data.MapObj[0].MapWidth)
              num7 -= self.game.Data.MapObj[0].MapWidth + 1;
            if (self.game.EditObj.HisHotX > num7)
              num6 = 1;
          }
        }
        else if (self.game.EditObj.HisHotX <= self.game.CornerX + 1)
          num6 = 1;
        if (self.game.EditObj.HisHotY <= self.game.CornerY + 1)
          num6 = 1;
        let mut num8: i32 =  self.game.CornerX + num4 - 2;
        if (self.game.Data.MapObj[0].MapLoop)
        {
          if (num8 > self.game.Data.MapObj[0].MapWidth)
            num8 -= self.game.Data.MapObj[0].MapWidth + 1;
          if (self.game.EditObj.HisHotX >= num8 && self.game.EditObj.HisHotX < self.game.CornerX - 5)
            num6 = 1;
        }
        else if (self.game.EditObj.HisHotX >= num8)
          num6 = 1;
        if (self.game.EditObj.HisHotY >= self.game.CornerY + num5 - 2)
          num6 = 1;
      }
      if (num3 == 1)
        self.game.EditObj.TempCoordList = CoordList::new();
      if (num6 != 1)
        return;
      if (self.game.Data.MapObj[0].MapLoop)
      {
        self.game.HandyFunctionsObj.SetcornerXY2(self.game.EditObj.HisHotX, self.game.EditObj.HisHotY);
      }
      else
      {
        self.game.EditObj.TempCoordList = CoordList::new();
        let mut num9: i32 =   Math.Round( self.game.EditObj.HisHotX -  num4 / 2.0);
        let mut num10: i32 =   Math.Round( self.game.EditObj.HisHotY -  num5 / 2.0);
        if (0 > num9)
          num9 = 0;
        if (0 > num10)
          num10 = 0;
        if (Math.Abs(self.game.CornerX - num9) > 3)
          self.game.CornerX = num9;
        if (Math.Abs(self.game.CornerY - num10) > 3)
          self.game.CornerY = num10;
        if (num9 == 0 & self.game.CornerX > 0)
          self.game.CornerX = 0;
        if (num10 == 0 & self.game.CornerY > 0)
          self.game.CornerY = 0;
        let mut num11: i32 =  265;
        if (self.game.EditObj.RealRound == 0)
          num11 += 100;
        let mut num12: i32 =   Math.Round( (self.game.ScreenWidth - 0) / 53.0);
        let mut num13: i32 =   Math.Round( (self.game.ScreenHeight - num11) / 48.0);
        let mut num14: i32 =  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth - self.game.CornerX;
        let mut num15: i32 =  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight - self.game.CornerY;
        if (num12 > num14)
          self.game.CornerX = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth - num12 + 2;
        if (num13 > num15)
          self.game.CornerY = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight - num13 + 1;
        if (self.game.CornerX < 0)
          self.game.CornerX = 0;
        if (self.game.CornerY >= 0)
          return;
        self.game.CornerY = 0;
      }
    }

    pub fn DoRefresh() => self.dostuff();

    pub fn dostuffonlyslider()
    {
    }

    pub fn dostuff()
    {
      if (self.Info1Id > 0)
        self.RemoveSubPart(self.Info1Id);
      if (self.info3id > 0)
        self.RemoveSubPart(self.info3id);
      if (self.info4id > 0)
        self.RemoveSubPart(self.info4id);
      if (self.info5id > 0)
        self.RemoveSubPart(self.info5id);
      if (self.info6id > 0)
        self.RemoveSubPart(self.info6id);
      if (self.SpecialTextId > 0)
        self.RemoveSubPart(self.SpecialTextId);
      if (self.ViewAntiCapId > 0)
        self.RemoveSubPart(self.ViewAntiCapId);
      if (self.ViewAntiCapTextId > 0)
        self.RemoveSubPart(self.ViewAntiCapTextId);
      if (self.ViewHistoryId > 0)
        self.RemoveSubPart(self.ViewHistoryId);
      if (self.ViewHistoryTextId > 0)
        self.RemoveSubPart(self.ViewHistoryTextId);
      if (self.ViewAntiCap2Id > 0)
        self.RemoveSubPart(self.ViewAntiCap2Id);
      if (self.ViewAntiCapText2Id > 0)
        self.RemoveSubPart(self.ViewAntiCapText2Id);
      if (self.ViewAntiCap3Id > 0)
        self.RemoveSubPart(self.ViewAntiCap3Id);
      if (self.ViewAntiCapText3Id > 0)
        self.RemoveSubPart(self.ViewAntiCapText3Id);
      if (self.OptionsListId > 0)
        self.RemoveSubPart(self.OptionsListId);
      if (self.OptionsList2Id > 0)
        self.RemoveSubPart(self.OptionsList2Id);
      if (self.ViewSupplyId > 0)
        self.RemoveSubPart(self.ViewSupplyId);
      if (self.ViewSupplyTextId > 0)
        self.RemoveSubPart(self.ViewSupplyTextId);
      self.NewBackGroundAndClearAll(self.w, self.h, self.game.MARCBOTBAR);
      self.ClearMouse();
      let mut num1: i32 =   Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      if (self.mapid > 0)
        self.RemoveSubPart(self.mapid);
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawBlockGradient2( Expression, 0, 0, self.w, self.h, Color.FromArgb(20, 0, 0, 0), Color.FromArgb(150, 0, 0, 0));
      DrawMod.DrawBlockGradient2( Expression, num1, 4, 1024, self.h - 6, self.game.MarcCol1, self.game.MarcCol2);
      DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  Expression, num1, 4, 1024, self.h - 6, num1, 4);
      SizeF sizeF1 = SizeF::new();
      if ((self.game.AIRunning | self.game.AIThreadRunning) & self.Curstep >= self.EndStep)
      {
        if (!Information.IsNothing( self.game.EditObj.TempAIString) & self.game.EditObj.AIProgressMax > 0)
        {
          let mut tsubpart: SubPartClass =  TextPartClass::new(self.game.EditObj.TempAIString, Font::new(self.game.FontCol.Families[1], 19f, FontStyle.Bold, GraphicsUnit.Pixel), 600, 20, true, tBlackBack: true, tProgress: ( Math.Round( self.game.EditObj.AIProgressNow /  self.game.EditObj.AIProgressMax * 100.0)), tMarc: true);
          self.info5id = self.AddSubPart( tsubpart, num1 + 200, 92, 600, 20, 0);
        }
      }
      else if (self.game.EditObj.HisLossCounter > -1)
      {
        self.OptionsListObj = ListClass::new();
        self.OptionsListObj.add("TYPE", -1, "START", "LOSS", tWeight: 9999999);
        try
        {
          let mut hisLossCounter: i32 =  self.game.EditObj.HisLossCounter;
          for (let mut index1: i32 =  0; index1 <= hisLossCounter; index1 += 1)
          {
            if (self.game.EditObj.HisLossAttacker[index1] == 1)
            {
              let mut index2: i32 =  self.game.EditObj.HisLossSFType[index1];
              if (index2 == -1)
              {
                self.OptionsListObj.add("Unknown Troops", -1, "?", "?", tWeight: 0);
              }
              else
              {
                let mut Number: i32 =  self.game.EditObj.HisLossOK[index1] + self.game.EditObj.HisLossDEAD[index1];
                if (self.game.Data.SFTypeObj[index2].Ratio > 0)
                  Number *= self.game.Data.SFTypeObj[index2].Ratio;
                let mut num2: i32 =  self.game.EditObj.HisLossDEAD[index1];
                if (self.game.Data.SFTypeObj[index2].Ratio > 0)
                  num2 *= self.game.Data.SFTypeObj[index2].Ratio;
                str: String = self.game.Data.SFTypeObj[index2].Name;
                if (Strings.Len(str) > 20)
                  str = Strings.Left(str, 20);
                if (Strings.Len(str) > 20)
                  str = Strings.Left(str, 20);
                self.OptionsListObj.add(str, -1, Strings.Trim(Conversion.Str( Number)), Strings.Trim(Conversion.Str( num2)), tWeight: num2);
              }
            }
          }
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          ProjectData.ClearProjectError();
        }
        str1: String = !(self.game.EditObj.HisLossAttReg < 0 | self.game.EditObj.HisLossAttReg > self.game.Data.RegimeCounter) ? "ATTACKER: " + Strings.UCase(self.game.Data.RegimeObj[self.game.EditObj.HisLossAttReg].Name) : "ATTACKER: ?";
        self.OptionsListObj.SortOnWeight(-1);
        SizeF sizeF2 = Expression.MeasureString(str1, self.game.MarcFont4);
        DrawMod.DrawTextColouredMarc( Expression, str1, self.game.MarcFont4,  Math.Round( ( (num1 + 200) +  (145.0 -  sizeF2.Width / 2.0))), 45, Color.White);
        let mut tsubpart1: SubPartClass =  new ListSubPartClass(self.OptionsListObj, 8, 290, -1, self.game, true, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 140, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (num1 + 200), bby: 65, tMarcStyle: true, overruleFont: ( self.game.MarcFont5));
        self.OptionsListId = self.AddSubPart( tsubpart1, num1 + 200, 65, 290, 176, 0);
        self.OptionsList2Obj = ListClass::new();
        self.OptionsList2Obj.add("TYPE", -1, "START", "LOSS", tWeight: 9999999);
        let mut hisLossCounter1: i32 =  self.game.EditObj.HisLossCounter;
        for (let mut index3: i32 =  0; index3 <= hisLossCounter1; index3 += 1)
        {
          if (self.game.EditObj.HisLossAttacker[index3] == 0)
          {
            let mut index4: i32 =  self.game.EditObj.HisLossSFType[index3];
            if (index4 == -1)
            {
              self.OptionsList2Obj.add("Unknown Troops", -1, "?", "?", tWeight: 0);
            }
            else
            {
              let mut Number: i32 =  self.game.EditObj.HisLossOK[index3] + self.game.EditObj.HisLossDEAD[index3];
              let mut num3: i32 =  self.game.EditObj.HisLossDEAD[index3];
              if (self.game.Data.SFTypeObj[index4].Ratio > 0)
                Number *= self.game.Data.SFTypeObj[index4].Ratio;
              if (self.game.Data.SFTypeObj[index4].Ratio > 0)
                num3 *= self.game.Data.SFTypeObj[index4].Ratio;
              str2: String = self.game.Data.SFTypeObj[index4].Name;
              if (Strings.Len(str2) > 20)
                str2 = Strings.Left(str2, 20);
              self.OptionsList2Obj.add(str2, -1, Strings.Trim(Conversion.Str( Number)), Strings.Trim(Conversion.Str( num3)), tWeight: num3);
            }
          }
        }
        self.OptionsList2Obj.SortOnWeight(-1);
        str3: String = !(self.game.EditObj.HisLossDefReg < 0 | self.game.EditObj.HisLossDefReg > self.game.Data.RegimeCounter) ? "DEFENDER: " + Strings.UCase(self.game.Data.RegimeObj[self.game.EditObj.HisLossDefReg].Name) : "DEFENDER: ?";
        SizeF sizeF3 = Expression.MeasureString(str3, self.game.MarcFont4);
        DrawMod.DrawTextColouredMarc( Expression, str3, self.game.MarcFont4,  Math.Round( ( (num1 + 500) +  (145.0 -  sizeF3.Width / 2.0))), 45, Color.White);
        let mut tsubpart2: SubPartClass =  new ListSubPartClass(self.OptionsList2Obj, 8, 290, -1, self.game, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 140, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (num1 + 500), bby: 65, tMarcStyle: true, overruleFont: ( self.game.MarcFont5));
        self.OptionsList2Id = self.AddSubPart( tsubpart2, num1 + 500, 65, 290, 176, 0);
        str4: String = "AN ATTACK";
        if (self.game.EditObj.HisAttackType == 2)
          str4 = "ATTACK".to_owned();
        if (self.game.EditObj.HisAttackType == 55)
          str4 = "AIR BRIDGE";
        if (self.game.EditObj.HisAttackType == 33)
          str4 = "AIR RECON";
        if (self.game.EditObj.HisAttackType == 12)
          str4 = "SEA ATTACK";
        if (self.game.EditObj.HisAttackType == 11)
          str4 = "ARTILLERY".to_owned();
        if (self.game.EditObj.HisAttackType == 13)
          str4 = "SEA ARTILLERY";
        if (self.game.EditObj.HisAttackType == 14)
          str4 = "AIRSTRIKE".to_owned();
        if (self.game.EditObj.HisAttackType == 15)
          str4 = "BOMBING RUN";
        if (self.game.EditObj.HisAttackType == 21)
          str4 = "AMPHIBIOUS ATTACK";
        if (self.game.EditObj.HisAttackType == 18)
          str4 = "AIR LANDING";
        if (self.game.EditObj.HisAttackType == 19)
          str4 = "PARADROP ATTACK";
        if (self.game.EditObj.HisAttackType == 42)
          str4 = "AIRLIFT".to_owned();
        if (self.game.EditObj.HisAttackType == 17)
          str4 = "ANTICAP DOGFIGHT";
        if (self.game.EditObj.HisAttackType == 31)
          str4 = "REBEL ATTACK (from inside Hex)";
        if (self.game.EditObj.HisRegimeWon == -1)
        {
          SizeF sizeF4 = Expression.MeasureString(str4, self.game.MarcFont3);
          DrawMod.DrawTextColouredMarc( Expression, str4, self.game.MarcFont3,  Math.Round( ( (num1 + 210) +  (300.0 -  sizeF4.Width / 2.0))), 7, Color.White);
          if (Strings.Len(self.game.EditObj.HisInfoString) > 0)
          {
            SizeF sizeF5 = Expression.MeasureString(self.game.EditObj.HisInfoString, self.game.MarcFont7);
            DrawMod.DrawTextColouredMarc( Expression, self.game.EditObj.HisInfoString, self.game.MarcFont7,  Math.Round( ( (num1 + 210) +  (300.0 -  sizeF5.Width / 2.0))), 28, Color.White);
          }
        }
        else
        {
          str5: String = self.game.EditObj.HisRegimeWon != self.game.EditObj.HisLossDefReg ? str4 + ": ATTACKER WAS VICTORIOUS" : str4 + ": DEFENDER STOOD FIRM";
          SizeF sizeF6 = Expression.MeasureString(str5, self.game.MarcFont3);
          DrawMod.DrawTextColouredMarc( Expression, str5, self.game.MarcFont3,  Math.Round( ( (num1 + 195) +  (300.0 -  sizeF6.Width / 2.0))), 7, Color.White);
          if (Strings.Len(self.game.EditObj.HisInfoString) > 0)
          {
            SizeF sizeF7 = Expression.MeasureString(self.game.EditObj.HisInfoString, self.game.MarcFont7);
            DrawMod.DrawTextColouredMarc( Expression, self.game.EditObj.HisInfoString, self.game.MarcFont7,  Math.Round( ( (num1 + 195) +  (300.0 -  sizeF7.Width / 2.0))), 28, Color.White);
          }
        }
      }
      else if (Strings.Len(self.game.EditObj.HisInfoString) > 0)
      {
        SizeF sizeF8 = Expression.MeasureString(self.game.EditObj.HisInfoString, self.game.MarcFont3);
        DrawMod.DrawTextColouredMarc( Expression, self.game.EditObj.HisInfoString, self.game.MarcFont3,  Math.Round( ( (num1 + 210) +  (300.0 -  sizeF8.Width / 2.0))), 92, Color.White);
      }
      else
      {
        str: String = "No further information on History Step";
        SizeF sizeF9 = Expression.MeasureString(str, self.game.MarcFont3);
        DrawMod.DrawTextColouredMarc( Expression, str, self.game.MarcFont3,  Math.Round( ( (num1 + 210) +  (300.0 -  sizeF9.Width / 2.0))), 92, Color.White);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.game.Data.RegimeObj[self.game.EditObj.RealTurn].AI | self.HumanPlayer > -1)
        return windowReturnClass;
      try
      {
        switch (nr)
        {
          case 27:
            self.game.EditObj.TempCoordList = CoordList::new();
            if ( self.game.Data.RuleVar[839] == 0.0)
              windowReturnClass.AddCommand(3, 3);
            else
              windowReturnClass.AddCommand(3, 11);
            self.game.EditObj.OrderType = 0;
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          case 80:
            self.AutoPlay = !self.AutoPlay;
            windowReturnClass.SetFlag(true);
            windowReturnClass.AddCommand(4, 81);
            self.dostuff();
            break;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (!Information.IsNothing( self.game.se1Thread))
      {
        if (self.game.se1Thread.ThreadState == ThreadState.Stopped)
        {
          self.game.se1ThreadRunning = false;
          self.game.se1Running = false;
          self.game.se1Thread.Abort();
          self.game.se1Thread.Join();
        }
        else if (!self.game.se1Running & self.game.se1ThreadRunning)
        {
          self.game.se1ThreadRunning = false;
          self.game.se1Thread.Abort();
          self.game.se1Thread.Join();
        }
        else
          windowReturnClass.SetFlag(true);
      }
      let mut num1: i32 =  1;
      let mut index: i32 =  self.game.EditObj.RealTurn;
      bool flag1 = false;
      while (num1 == 1)
      {
        index += 1;
        num2: i32;
        if (index > self.game.Data.RegimeCounter)
        {
          index = 0;
          num2 += 1;
        }
        if (num2 > 1)
          num1 = 0;
        if (!self.game.Data.RegimeObj[index].Sleep)
        {
          flag1 = self.game.Data.RegimeObj[index].AI;
          num1 = 0;
        }
      }
      if (!self.game.Data.RegimeObj[self.game.EditObj.RealTurn].AI & self.HumanPlayer > -1)
        flag1 = false;
      self.game.EditObj.LastHistoryStep = -1;
      bool flag2;
      if (self.EndStep >= -1)
      {
        TimeSpan timeSpan = DateAndTime.Now.Subtract(self.showtime);
        if ( timeSpan.Ticks > 2000000.0 & self.game.EditObj.HisLossCounter == -1 | timeSpan.Ticks > 20000000L)
        {
          if (self.HumanPlayer > -1)
            self.EndStep = self.game.HandyFunctionsObj.GetRegimeHistoryTotSteps(self.HumanPlayer);
          self.showtime = DateAndTime.Now;
          if (self.AutoPlay & (self.Curstep < self.EndStep | self.EndStep == -1) | self.game.EditObj.AIMoving & !self.game.se1Running & !self.game.se1ThreadRunning & self.Curstep >= self.EndStep)
          {
            self.game.EditObj.TempCoordList = CoordList::new();
            if (self.Curstep < self.EndStep)
            {
              self.Forward(1);
              this += 1.Curstep;
              self.StartStep = 1;
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 81);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 9);
            }
            if (self.Curstep == self.EndStep & self.HumanPlayer == -1)
              self.AutoPlay = false;
            if (!flag1 & self.HumanPlayer > -1 & (self.Curstep >= self.EndStep | self.EndStep == -1) && !self.game.se1Running & !self.game.se1ThreadRunning)
            {
              self.game.EditObj.AIMoving = false;
              windowReturnClass.SetFlag(true);
              self.game.EditObj.Test = 99;
              windowReturnClass.AddCommand(3, 13);
              return windowReturnClass;
            }
            windowReturnClass.SetFlag(true);
            self.dostuff();
            return windowReturnClass;
          }
          if (self.game.se1Running)
          {
            windowReturnClass.SetFlag(true);
            self.dostuff();
            flag2 = false;
          }
        }
        else if ( timeSpan.Ticks > 2500000.0 & self.game.se1Running)
        {
          windowReturnClass.SetFlag(true);
          self.dostuff();
          flag2 = false;
        }
      }
      else
      {
        TimeSpan timeSpan = DateAndTime.Now.Subtract(self.showtime);
        if ( timeSpan.Ticks > 2500000.0 & self.game.se1Running)
        {
          self.EndStep = self.game.HandyFunctionsObj.GetRegimeHistoryTotSteps(self.HumanPlayer);
          windowReturnClass.SetFlag(true);
          self.showtime = DateAndTime.Now;
          if (self.EndStep != self.lastendstep)
            windowReturnClass.AddCommand(4, 81);
          flag2 = true;
        }
        else if (timeSpan.Ticks > 10000000L & self.AutoPlay & !self.game.se1Running & (self.HumanPlayer == -1 | !self.game.se1ThreadRunning))
        {
          windowReturnClass.SetFlag(true);
          self.showtime = DateAndTime.Now;
          if (self.EndStep != self.lastendstep)
            windowReturnClass.AddCommand(4, 81);
          flag2 = true;
        }
      }
      self.lastendstep = self.EndStep;
      if (flag2)
      {
        windowReturnClass.Flag = true;
        self.dostuff();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  self.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num1: i32 =  self.SubPartID[index];
            num2: i32;
            if (num1 == self.OptionsListId)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == self.OptionsList2Id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
            }
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
