// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.HistoryWindowClass
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
  pub class HistoryWindowClass : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     info3id: i32;
     mapid: i32;
     info4id: i32;
     info5id: i32;
     info6id: i32;
     ExitId: i32;
     ListClass MapListObj;
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
     StartStep: i32;
     EndStep: i32;
     TotStep: i32;
     Curstep: i32;
     RealStepNr: i32;
     DateTime showtime;
     OptionsListId: i32;
     ATListClass OptionsListObj;
     OptionsList2Id: i32;
     ATListClass OptionsList2Obj;
     MiniMapId: i32;
     AutoPlayID: i32;
     bool AutoPlay;
     SpecialId: i32;
     SpecialTextId: i32;
     detail1: i32;
     detail2: i32;
     HumanPlayer: i32;
     Turny: i32;
     detailnr: i32;
     SkipId: i32;
     lastregime: i32;
     lastendstep: i32;
     bool writing;

    pub HistoryWindowClass( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base( tGame, tGame.ScreenWidth, 250, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.dostuff();
      this.StartStep = -1;
      this.EndStep = 0;
      this.lastendstep = 0;
      this.TotStep = -1;
      this.Curstep = 0;
      this.HumanPlayer = -1;
      this.detailnr = this.game.EditObj.MapSelected;
      this.Turny = this.game.Data.Turn;
      this.game.EditObj.MiniMap = new Bitmap(1, 1);
      if (this.game.HandyFunctionsObj.GetHumanPlayers() == 1 & this.game.EditObj.TempAIWatch)
      {
        let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
        for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
        {
          if (!this.game.Data.RegimeObj[index].AI & !this.game.Data.RegimeObj[index].Sleep)
          {
            this.HumanPlayer = index;
            this.Turny = index;
            this.AutoPlay = true;
            this.game.EditObj.AIMoving = true;
            break;
          }
        }
      }
      this.game.EditObj.TempAIWatch = false;
      this.game.EditObj.HumanPlayer = this.HumanPlayer;
      this.game.EditObj.OrderType = 26;
      this.showtime = DateAndTime.Now;
      this.game.EditObj.HisInfoString = "";
      this.game.EditObj.HisLossCounter = -1;
      if (this.game.Data.RegimeObj[this.Turny].HistoryStepCounter > -1)
      {
        this.StartStep = 1;
        this.EndStep = this.game.HandyFunctionsObj.GetRegimeHistoryTotSteps(this.Turny);
      }
      this.StartSit();
      if (this.game.EditObj.LastHistoryStep > 0)
      {
        this.Forward(this.game.EditObj.LastHistoryStep);
        this.Curstep = this.game.EditObj.LastHistoryStep;
        this.game.EditObj.LastHistoryStep = -1;
      }
      this.dostuff();
    }

    pub fn StartSit()
    {
      this.game.EditObj.HisForce = new MapMatrix2[this.game.Data.MapCounter + 1];
      this.game.EditObj.HisSFType = new MapMatrix2[this.game.Data.MapCounter + 1];
      this.game.EditObj.HisOwner = new MapMatrix2[this.game.Data.MapCounter + 1];
      this.game.EditObj.HisHis = new MapMatrix2[this.game.Data.MapCounter + 1];
      this.game.EditObj.HisDepth = new MapMatrix2[this.game.Data.MapCounter + 1];
      let mut mapCounter: i32 =  this.game.Data.MapCounter;
      for (let mut index: i32 =  0; index <= mapCounter; index += 1)
      {
        this.game.EditObj.HisForce[index] = this.game.Data.RegimeObj[this.Turny].HistoryForce[index].Clone();
        this.game.EditObj.HisSFType[index] = this.game.Data.RegimeObj[this.Turny].HistorySFType[index].Clone();
        this.game.EditObj.HisOwner[index] = this.game.Data.RegimeObj[this.Turny].HistoryOwner[index].Clone();
        this.game.EditObj.HisHis[index] = this.game.Data.RegimeObj[this.Turny].HistoryHis[index].Clone();
        this.game.EditObj.HisDepth[index] = this.game.Data.RegimeObj[this.Turny].HistoryDepth[index].Clone();
      }
      if (this.game.Data.RegimeObj[this.Turny].HistoryStepCounter > -1)
        this.RealStepNr = this.game.Data.RegimeObj[this.Turny].HistoryStep[0].StepNr - 1;
      this.game.EditObj.HisHotX = -1;
      this.game.EditObj.HisHotY = -1;
      this.game.EditObj.HisHotMap = -1;
      this.game.EditObj.HisLossCounter = -1;
      this.game.EditObj.HisLossAttacker = new int[1];
      this.game.EditObj.HisLossDEAD = new int[1];
      this.game.EditObj.HisLossOK = new int[1];
      this.game.EditObj.HisLossSFType = new int[1];
      this.game.EditObj.HisRegimeWon = -1;
      this.game.EditObj.HisAttackType = -1;
      this.game.EditObj.TempCoordList = CoordList::new();
    }

    pub fn Forward(steps: i32)
    {
      let mut num1: i32 =  -1;
      let mut num2: i32 =  0;
      this.game.EditObj.HisHotX = -1;
      this.game.EditObj.HisHotY = -1;
      this.game.EditObj.HisHotMap = -1;
      this.game.EditObj.HisLossAttacker = new int[1];
      this.game.EditObj.HisLossDEAD = new int[1];
      this.game.EditObj.HisLossOK = new int[1];
      this.game.EditObj.HisLossSFType = new int[1];
      this.game.EditObj.HisLossCounter = -1;
      this.game.EditObj.HisRegimeWon = -1;
      this.game.EditObj.HisLossDefReg = -1;
      this.game.EditObj.HisLossAttReg = -1;
      this.game.EditObj.HisInfoString = "";
      this.game.EditObj.HisAttackType = -1;
      this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
      let mut num3: i32 =  0;
      let mut historyStepCounter: i32 =  this.game.Data.RegimeObj[this.Turny].HistoryStepCounter;
      for (let mut index1: i32 =  0; index1 <= historyStepCounter; index1 += 1)
      {
        HistoryStepClass historyStepClass = this.game.Data.RegimeObj[this.Turny].HistoryStep[index1];
        if (historyStepClass.StepNr > this.RealStepNr)
        {
          if (historyStepClass.StepNr != num1)
          {
            num2 += 1;
            num1 = historyStepClass.StepNr;
            if (num2 > steps)
            {
              this.RealStepNr = historyStepClass.StepNr - 1;
              break;
            }
            this.game.EditObj.HisLossCounter = -1;
            this.game.EditObj.HisRegimeWon = -1;
            this.game.EditObj.HisLossDefReg = -1;
            this.game.EditObj.HisLossAttReg = -1;
            this.game.EditObj.HisInfoString = "";
            this.game.EditObj.HisAttackType = -1;
          }
          else if (num2 > steps)
          {
            this.RealStepNr = historyStepClass.StepNr - 1;
            break;
          }
          this.lastregime = historyStepClass.Regime;
          this.game.EditObj.HisForce[historyStepClass.Map].Value[historyStepClass.X, historyStepClass.Y] = historyStepClass.Force;
          this.game.EditObj.HisSFType[historyStepClass.Map].Value[historyStepClass.X, historyStepClass.Y] = historyStepClass.SFType;
          this.game.EditObj.HisOwner[historyStepClass.Map].Value[historyStepClass.X, historyStepClass.Y] = historyStepClass.Ownership;
          this.game.EditObj.HisHis[historyStepClass.Map].Value[historyStepClass.X, historyStepClass.Y] = historyStepClass.His;
          this.game.EditObj.HisDepth[historyStepClass.Map].Value[historyStepClass.X, historyStepClass.Y] = historyStepClass.Depth;
          this.game.EditObj.TempCoordList.AddCoord(historyStepClass.X, historyStepClass.Y, historyStepClass.Map);
          let mut tfacing: i32 =  1;
          do
          {
            Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(historyStepClass.X, historyStepClass.Y, historyStepClass.Map, tfacing);
            if (coordinate.onmap)
              this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
            tfacing += 1;
          }
          while (tfacing <= 6);
          this.game.SelectX = historyStepClass.X;
          this.game.SelectY = historyStepClass.Y;
          if (this.game.EditObj.MapSelected != historyStepClass.Map)
          {
            this.game.EditObj.MapSelected = historyStepClass.Map;
            this.game.CornerX = 0;
            this.game.CornerY = 0;
            num3 = 1;
          }
          if (Strings.Len(historyStepClass.InfoString) > 0)
            this.game.EditObj.HisInfoString = historyStepClass.InfoString;
          if (historyStepClass.AttackOtherType > -1)
          {
            this.game.EditObj.HisHotX = historyStepClass.X;
            this.game.EditObj.HisHotY = historyStepClass.Y;
            this.game.EditObj.HisHotMap = historyStepClass.Map;
            this.game.EditObj.HisNeighbour = Neighbours::new();
            let mut index2: i32 =  0;
            do
            {
              this.game.EditObj.HisNeighbour.data[index2] = historyStepClass.AttackDirection[index2];
              index2 += 1;
            }
            while (index2 <= 5);
            this.game.EditObj.HisAttackType = historyStepClass.AttackOtherType;
          }
          if (historyStepClass.LossCounter > -1)
          {
            this.game.EditObj.HisLossAttacker = new int[historyStepClass.LossCounter + 1];
            this.game.EditObj.HisLossDEAD = new int[historyStepClass.LossCounter + 1];
            this.game.EditObj.HisLossOK = new int[historyStepClass.LossCounter + 1];
            this.game.EditObj.HisLossSFType = new int[historyStepClass.LossCounter + 1];
            this.game.EditObj.HisLossCounter = historyStepClass.LossCounter;
            let mut lossCounter: i32 =  historyStepClass.LossCounter;
            for (let mut index3: i32 =  0; index3 <= lossCounter; index3 += 1)
            {
              this.game.EditObj.HisLossAttacker[index3] = historyStepClass.LossAttacker[index3];
              this.game.EditObj.HisLossDEAD[index3] = historyStepClass.LossDEAD[index3];
              this.game.EditObj.HisLossOK[index3] = historyStepClass.LossOK[index3];
              this.game.EditObj.HisLossSFType[index3] = historyStepClass.LossSFType[index3];
            }
            this.game.EditObj.HisRegimeWon = historyStepClass.LossRegimeWin;
            this.game.EditObj.HisLossAttReg = historyStepClass.LossAttReg;
            this.game.EditObj.HisLossDefReg = historyStepClass.LossDefReg;
          }
        }
      }
      if (this.game.EditObj.HisHotX > -1 & this.game.EditObj.HisHotY > -1)
      {
        this.game.SelectX = this.game.EditObj.HisHotX;
        this.game.SelectY = this.game.EditObj.HisHotY;
        this.game.EditObj.MapSelected = this.game.EditObj.HisHotMap;
      }
      let mut num4: i32 =   Math.Round( this.game.ScreenWidth / 53.0);
      let mut num5: i32 =   Math.Round( (this.game.ScreenHeight - 200) / 48.0);
      let mut num6: i32 =  0;
      if (this.game.SelectX <= this.game.CornerX + 1)
        num6 = 1;
      if (this.game.SelectY <= this.game.CornerY + 1)
        num6 = 1;
      if (this.game.SelectX >= this.game.CornerX + num4 - 2)
        num6 = 1;
      if (this.game.SelectY >= this.game.CornerY + num5 - 2)
        num6 = 1;
      if (num3 == 1)
        this.game.EditObj.TempCoordList = CoordList::new();
      if (num6 != 1)
        return;
      this.game.EditObj.TempCoordList = CoordList::new();
      let mut num7: i32 =   Math.Round( this.game.SelectX -  num4 / 2.0);
      let mut num8: i32 =   Math.Round( this.game.SelectY -  num5 / 2.0);
      if (0 > num7)
        num7 = 0;
      if (0 > num8)
        num8 = 0;
      if (Math.Abs(this.game.CornerX - num7) > 3)
        this.game.CornerX = num7;
      if (Math.Abs(this.game.CornerY - num8) > 3)
        this.game.CornerY = num8;
      if (num7 == 0 & this.game.CornerX > 0)
        this.game.CornerX = 0;
      if (num8 == 0 & this.game.CornerY > 0)
        this.game.CornerY = 0;
      let mut num9: i32 =  265;
      if (this.game.Data.Round == 0)
        num9 += 100;
      let mut num10: i32 =   Math.Round( (this.game.ScreenWidth - 0) / 53.0);
      let mut num11: i32 =   Math.Round( (this.game.ScreenHeight - num9) / 48.0);
      let mut num12: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth - this.game.CornerX;
      let mut num13: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight - this.game.CornerY;
      if (num10 > num12)
        this.game.CornerX = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth - num10 + 2;
      if (num11 > num13)
        this.game.CornerY = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight - num11 + 1;
      if (this.game.CornerX < 0)
        this.game.CornerX = 0;
      if (this.game.CornerY >= 0)
        return;
      this.game.CornerY = 0;
    }

    pub fn DoRefresh() => this.dostuff();

    pub fn dostuffonlyslider()
    {
      this.writing = true;
      let mut num1: i32 =   Math.Round( (this.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 =  5;
      if (this.Slider1Id > 0)
        this.RemoveSubPart(this.Slider1Id);
      if (this.ExitId > 0)
        this.RemoveSubPart(this.ExitId);
      if (this.StartStep > -1)
      {
        let mut tsubpart: SubPartClass =  new NumberSliderSubPartClass2(this.game, "Step", " of " + Conversion.Str( Math.Max(this.Curstep, this.EndStep)), 340, 0, this.EndStep, this.Curstep, tbackbitmap: ( this.OwnBitmap), bbx: (num1 + 5), bby: (num2 + 50));
        this.Slider1Id = this.AddSubPart( tsubpart, num1 + 5, num2 + 50, 340, 40, 0);
      }
      if (this.HumanPlayer == -1 | !this.game.AIThreadRunning)
      {
        let mut tsubpart: SubPartClass =  new SteveButtonPartClass(this.game.BACKBUTTON, tBackbitmap: ( this.OwnBitmap), bbx: (num1 + 5), bby: (num2 + 10));
        this.ExitId = this.AddSubPart( tsubpart, num1 + 5, num2 + 10, 35, 35, 1);
      }
      this.writing = false;
    }

    pub fn dostuff()
    {
      this.writing = true;
      if (this.Info1Id > 0)
        this.RemoveSubPart(this.Info1Id);
      if (this.info2id > 0)
        this.RemoveSubPart(this.info2id);
      if (this.info3id > 0)
        this.RemoveSubPart(this.info3id);
      if (this.info4id > 0)
        this.RemoveSubPart(this.info4id);
      if (this.info5id > 0)
        this.RemoveSubPart(this.info5id);
      if (this.info6id > 0)
        this.RemoveSubPart(this.info6id);
      if (this.SpecialId > 0)
        this.RemoveSubPart(this.SpecialId);
      if (this.SpecialTextId > 0)
        this.RemoveSubPart(this.SpecialTextId);
      if (this.Slider1Id > 0)
        this.RemoveSubPart(this.Slider1Id);
      if (this.ExitId > 0)
        this.RemoveSubPart(this.ExitId);
      if (this.ViewAntiCapId > 0)
        this.RemoveSubPart(this.ViewAntiCapId);
      if (this.ViewAntiCapTextId > 0)
        this.RemoveSubPart(this.ViewAntiCapTextId);
      if (this.ViewHistoryId > 0)
        this.RemoveSubPart(this.ViewHistoryId);
      if (this.ViewHistoryTextId > 0)
        this.RemoveSubPart(this.ViewHistoryTextId);
      if (this.ViewAntiCap2Id > 0)
        this.RemoveSubPart(this.ViewAntiCap2Id);
      if (this.ViewAntiCapText2Id > 0)
        this.RemoveSubPart(this.ViewAntiCapText2Id);
      if (this.ViewAntiCap3Id > 0)
        this.RemoveSubPart(this.ViewAntiCap3Id);
      if (this.ViewAntiCapText3Id > 0)
        this.RemoveSubPart(this.ViewAntiCapText3Id);
      if (this.OptionsListId > 0)
        this.RemoveSubPart(this.OptionsListId);
      if (this.OptionsList2Id > 0)
        this.RemoveSubPart(this.OptionsList2Id);
      if (this.ViewSupplyId > 0)
        this.RemoveSubPart(this.ViewSupplyId);
      if (this.ViewSupplyTextId > 0)
        this.RemoveSubPart(this.ViewSupplyTextId);
      if (this.MiniMapId > 0)
        this.RemoveSubPart(this.MiniMapId);
      if (this.AutoPlayID > 0)
        this.RemoveSubPart(this.AutoPlayID);
      if (this.SkipId > 0)
        this.RemoveSubPart(this.SkipId);
      if (this.mapid > 0)
        this.RemoveSubPart(this.mapid);
      this.NewBackGroundAndClearAll(this.game.ScreenWidth, 250, -1);
      Graphics objGraphics = Graphics.FromImage((Image) this.OwnBitmap);
      let mut num1: i32 =   Math.Round( (this.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 =  5;
      DrawMod.DrawBlock( objGraphics, 0, 0, this.game.ScreenWidth, 40,  this.game.VicColor5.R,  this.game.VicColor5.G,  this.game.VicColor5.B,  this.game.VicColor5.A);
      DrawMod.DrawBlock( objGraphics, 0, 40, this.game.ScreenWidth, 10,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
      DrawMod.drawLine( objGraphics, 0, 50, this.game.ScreenWidth, 50,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
      this.game.EditObj.HumanPlayer = this.HumanPlayer;
      let mut tsubpart1: SubPartClass =  new MiniMapPartClass(this.game, ty: 130, ZoomLevel: 0, humanplayer: this.HumanPlayer);
      this.MiniMapId = this.AddSubPart( tsubpart1, num1 + 5, num2 + 102, 200, 130, 0);
      DrawMod.DrawRectangle( objGraphics, num1 + 4, num2 + 101, 201, 131,  DrawMod.TGame.VicColor3.R,  DrawMod.TGame.VicColor3.G,  DrawMod.TGame.VicColor3.B,  DrawMod.TGame.VicColor3.A);
      if (this.game.Data.MapCounter > 0)
      {
        let mut num3: i32 =  -1;
        let mut num4: i32 =  -1;
        this.MapListObj = ListClass::new();
        let mut mapCounter: i32 =  this.game.Data.MapCounter;
        for (let mut tdata: i32 =  0; tdata <= mapCounter; tdata += 1)
        {
          let mut num5: i32 =  0;
          if (this.game.Data.Round == 0)
            num5 = 1;
          if (this.game.Data.MapObj[tdata].CanSee)
            num5 = 1;
          if (!this.game.Data.ShrowdOn)
            num5 = 1;
          if (num5 == 1)
          {
            num4 += 1;
            this.MapListObj.add(this.game.Data.MapObj[tdata].Name, tdata);
            if (this.game.EditObj.MapSelected == tdata)
              num3 = num4;
          }
        }
        ListClass mapListObj = this.MapListObj;
        let mut tlistselect: i32 =  num3;
        let mut game: GameClass = this.game;
         local1: Bitmap =  this.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        tsubpart1 =  new ListSubPartClass(mapListObj, 6, 180, tlistselect, game, tHeader: "Maps", tbackbitmap: ( local1), bbx: 220, bby: 95, overruleFont: ( local2));
        this.mapid = this.AddSubPart( tsubpart1, 220, 95, 120, 144, 0);
      }
      if (this.HumanPlayer == -1 | !this.game.AIThreadRunning)
      {
        tsubpart1 =  new SteveButtonPartClass(this.game.BACKBUTTON, tBackbitmap: ( this.OwnBitmap), bbx: 3, bby: 3);
        this.ExitId = this.AddSubPart( tsubpart1, 3, 3, 35, 35, 1);
      }
      if (this.game.AIRunning | this.game.AIThreadRunning)
      {
        tsubpart1 =  new ATTextPartClass("AI " + this.game.Data.RegimeObj[this.game.Data.Turn].Name + " is thinking and moving", this.game.VicFont1, 350, 27, true, toutline: true);
        this.info2id = this.AddSubPart( tsubpart1, num1 + 350, 6, 325, 27, 0);
      }
      else if (!this.game.AIRunning & !this.game.AIThreadRunning & this.game.EditObj.AIMoving)
      {
        tsubpart1 =  new ATTextPartClass("AI has finished its turn", this.game.VicFont1, 350, 27, true, toutline: true);
        this.info2id = this.AddSubPart( tsubpart1, num1 + 350, 6, 325, 27, 0);
      }
      else
      {
        tsubpart1 =  new ATTextPartClass("History Screen", this.game.VicFont1, 350, 27, true);
        this.info2id = this.AddSubPart( tsubpart1, num1 + 350, 6, 325, 27, 0);
      }
      if (this.StartStep > -1)
      {
        tsubpart1 =  new NumberSliderSubPartClass2(this.game, "Step", " of " + Conversion.Str( Math.Max(this.Curstep, this.EndStep)), 365, 0, this.EndStep, this.Curstep, tbackbitmap: ( this.OwnBitmap), bbx: (num1 + 5), bby: (num2 + 50));
        this.Slider1Id = this.AddSubPart( tsubpart1, num1 + 5, num2 + 50, 365, 40, 0);
      }
      if (this.Curstep == 0)
      {
        tsubpart1 =  new ATTextPartClass("Viewing History, Use Slider to view", this.game.VicFont2, 600, 192, true, tBlackBack: true);
        this.info3id = this.AddSubPart( tsubpart1, num1 + 400, num2 + 49, 600, 192, 0);
      }
      tsubpart1 =  new TextButtonPartClass("Back", 150, "Click to start history again at step 0",  this.OwnBitmap, num1 + 220, num2 + 132, theight: 30);
      this.Info1Id = this.AddSubPart( tsubpart1, num1 + 220, num2 + 132, 150, 30, 1);
      tsubpart1 =  new TextButtonPartClass("Next Battle", 150, "Click to forward to the next step in history where a battle is reported",  this.OwnBitmap, num1 + 220, num2 + 164, theight: 30);
      this.SpecialId = this.AddSubPart( tsubpart1, num1 + 220, num2 + 164, 150, 30, 1);
      tsubpart1 =  new TextButtonPartClass("Next Regime", 150, "Click to forward to the next step in history caused by another regime",  this.OwnBitmap, num1 + 220, num2 + 196, theight: 30);
      this.SkipId = this.AddSubPart( tsubpart1, num1 + 220, num2 + 196, 150, 30, 1);
      if (!this.AutoPlay)
      {
        tsubpart1 =  new TextButtonPartClass("Play", 150, "Click to start Autoplay [shortkey P]",  this.OwnBitmap, num1 + 220, num2 + 100, theight: 30);
        this.AutoPlayID = this.AddSubPart( tsubpart1, num1 + 220, num2 + 100, 150, 30, 1);
      }
      else
      {
        tsubpart1 =  new TextButtonPartClass("Stop", 150, "Click to pause Autoplay [shortkey P]",  this.OwnBitmap, num1 + 220, num2 + 100, theight: 30);
        this.AutoPlayID = this.AddSubPart( tsubpart1, num1 + 220, num2 + 100, 150, 30, 1);
      }
      if (this.game.EditObj.HisLossCounter > -1)
      {
        try
        {
          this.OptionsListObj = ATListClass::new();
          this.OptionsListObj.add("TYPE", -1, "START", "LOSS");
          let mut hisLossCounter: i32 =  this.game.EditObj.HisLossCounter;
          for (let mut index1: i32 =  0; index1 <= hisLossCounter; index1 += 1)
          {
            if (this.game.EditObj.HisLossAttacker[index1] == 1)
            {
              let mut index2: i32 =  this.game.EditObj.HisLossSFType[index1];
              let mut Number1: i32 =  this.game.EditObj.HisLossOK[index1] + this.game.EditObj.HisLossDEAD[index1];
              if (this.game.Data.SFTypeObj[index2].Ratio > 0)
                Number1 *= this.game.Data.SFTypeObj[index2].Ratio;
              let mut Number2: i32 =  this.game.EditObj.HisLossDEAD[index1];
              if (this.game.Data.SFTypeObj[index2].Ratio > 0)
                Number2 *= this.game.Data.SFTypeObj[index2].Ratio;
              this.OptionsListObj.add(this.game.Data.SFTypeObj[index2].Name, -1, Strings.Trim(Conversion.Str( Number1)), Strings.Trim(Conversion.Str( Number2)));
            }
          }
          tsubpart1 =  new ATListSubPartClass(this.OptionsListObj, 7, 290, -1, this.game, true, "ATTACKER: " + Strings.UCase(this.game.Data.RegimeObj[this.game.EditObj.HisLossAttReg].Name), tShowPair: true, tValueWidth: 140, tbackbitmap: ( this.OwnBitmap), bbx: (num1 + 400), bby: (num2 + 90));
          this.OptionsListId = this.AddSubPart( tsubpart1, num1 + 400, num2 + 90, 290, 160, 0);
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          ProjectData.ClearProjectError();
        }
        try
        {
          this.OptionsList2Obj = ATListClass::new();
          this.OptionsList2Obj.add("TYPE", -1, "START", "LOSS");
          let mut hisLossCounter: i32 =  this.game.EditObj.HisLossCounter;
          for (let mut index3: i32 =  0; index3 <= hisLossCounter; index3 += 1)
          {
            if (this.game.EditObj.HisLossAttacker[index3] == 0)
            {
              let mut index4: i32 =  this.game.EditObj.HisLossSFType[index3];
              let mut Number3: i32 =  this.game.EditObj.HisLossOK[index3] + this.game.EditObj.HisLossDEAD[index3];
              let mut Number4: i32 =  this.game.EditObj.HisLossDEAD[index3];
              if (this.game.Data.SFTypeObj[index4].Ratio > 0)
                Number3 *= this.game.Data.SFTypeObj[index4].Ratio;
              if (this.game.Data.SFTypeObj[index4].Ratio > 0)
                Number4 *= this.game.Data.SFTypeObj[index4].Ratio;
              this.OptionsList2Obj.add(this.game.Data.SFTypeObj[index4].Name, -1, Strings.Trim(Conversion.Str( Number3)), Strings.Trim(Conversion.Str( Number4)));
            }
          }
          if (this.game.EditObj.HisLossDefReg < 0)
          {
            let mut tsubpart2: SubPartClass =  new ATListSubPartClass(this.OptionsList2Obj, 7, 290, -1, this.game, true, "DEFENDER: NEUTRAL?", tShowPair: true, tValueWidth: 140, tbackbitmap: ( this.OwnBitmap), bbx: (num1 + 710), bby: (num2 + 90));
            this.OptionsList2Id = this.AddSubPart( tsubpart2, num1 + 710, num2 + 90, 290, 160, 0);
          }
          else
          {
            let mut tsubpart3: SubPartClass =  new ATListSubPartClass(this.OptionsList2Obj, 7, 290, -1, this.game, true, "DEFENDER: " + Strings.UCase(this.game.Data.RegimeObj[this.game.EditObj.HisLossDefReg].Name), tShowPair: true, tValueWidth: 140, tbackbitmap: ( this.OwnBitmap), bbx: (num1 + 710), bby: (num2 + 90));
            this.OptionsList2Id = this.AddSubPart( tsubpart3, num1 + 710, num2 + 90, 290, 160, 0);
          }
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          ProjectData.ClearProjectError();
        }
        txt: String = "AN ATTACK";
        if (this.game.EditObj.HisAttackType == 2)
          txt = "LAND ATTACK";
        if (this.game.EditObj.HisAttackType == 12)
          txt = "SEA ATTACK";
        if (this.game.EditObj.HisAttackType == 11)
          txt = "LAND ARTILLERY";
        if (this.game.EditObj.HisAttackType == 13)
          txt = "SEA ARTILLERY";
        if (this.game.EditObj.HisAttackType == 14)
          txt = "AIRSTRIKE".to_owned();
        if (this.game.EditObj.HisAttackType == 15)
          txt = "BOMBING RUN";
        if (this.game.EditObj.HisAttackType == 21)
          txt = "AMPHIBIOUS ATTACK";
        if (this.game.EditObj.HisAttackType == 19)
          txt = "PARADROP ATTACK";
        if (this.game.EditObj.HisAttackType == 42)
          txt = "AIRLIFT".to_owned();
        if (this.game.EditObj.HisAttackType == 17)
          txt = "ANTICAP DOGFIGHT";
        if (this.game.EditObj.HisAttackType == 31)
          txt = "ATTACK".to_owned();
        if (this.game.EditObj.HisRegimeWon == -1)
        {
          let mut tsubpart4: SubPartClass =  new ATTextPartClass(txt, this.game.VicFont1, 600, 27, true, tBlackBack: true);
          this.info5id = this.AddSubPart( tsubpart4, num1 + 400, num2 + 49, 600, 27, 0);
          if (Strings.Len(this.game.EditObj.HisInfoString) > 0)
          {
            let mut tsubpart5: SubPartClass =  new ATTextPartClass(this.game.EditObj.HisInfoString, this.game.VicFont3, 600, 12, true, tBlackBack: true);
            this.info6id = this.AddSubPart( tsubpart5, num1 + 400, num2 + 75, 600, 12, 0);
          }
        }
        else
        {
          let mut tsubpart6: SubPartClass =  new ATTextPartClass(this.game.EditObj.HisRegimeWon != this.game.EditObj.HisLossDefReg ? txt + " ATTACKER WAS VICTORIOUS" : txt + ": DEFENDER STOOD FIRM", this.game.VicFont1, 600, 27, true, tBlackBack: true);
          this.info5id = this.AddSubPart( tsubpart6, num1 + 400, num2 + 49, 600, 27, 0);
          if (Strings.Len(this.game.EditObj.HisInfoString) > 0)
          {
            let mut tsubpart7: SubPartClass =  new ATTextPartClass(this.game.EditObj.HisInfoString, this.game.VicFont3, 600, 12, true, tBlackBack: true);
            this.info6id = this.AddSubPart( tsubpart7, num1 + 400, num2 + 75, 600, 12, 0);
          }
        }
      }
      else if (Strings.Len(this.game.EditObj.HisInfoString) > 0)
      {
        tsubpart1 =  new ATTextPartClass(this.game.EditObj.HisInfoString, this.game.VicFont1, 600, 192, true, tBlackBack: true);
        this.info5id = this.AddSubPart( tsubpart1, num1 + 400, num2 + 49, 600, 192, 0);
      }
      if (!Information.IsNothing( objGraphics))
      {
        objGraphics.Dispose();
        objGraphics = (Graphics) null;
      }
      this.writing = false;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (nr == 80)
      {
        this.AutoPlay = !this.AutoPlay;
        windowReturnClass.SetFlag(true);
        this.dostuff();
        return windowReturnClass;
      }
      if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
        return windowReturnClass;
      try
      {
        if (nr == 27)
        {
          this.game.EditObj.TempCoordList = CoordList::new();
          if ( this.game.Data.RuleVar[839] == 0.0)
            windowReturnClass.AddCommand(3, 3);
          else
            windowReturnClass.AddCommand(3, 11);
          this.game.EditObj.OrderType = 0;
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

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (!Information.IsNothing( this.game.AIThread))
      {
        if (this.game.AIThread.ThreadState == ThreadState.Stopped)
        {
          this.game.AIThreadRunning = false;
          this.game.AIRunning = false;
          this.game.AIThread.Abort();
          this.game.AIThread.Join();
        }
        else if (!this.game.AIRunning & this.game.AIThreadRunning)
        {
          this.game.AIThreadRunning = false;
          this.game.AIThread.Abort();
          this.game.AIThread.Join();
        }
        else
          windowReturnClass.SetFlag(true);
      }
      let mut num1: i32 =  1;
      let mut index: i32 =  this.game.Data.Turn;
      bool flag1 = false;
      while (num1 == 1)
      {
        index += 1;
        num2: i32;
        if (index > this.game.Data.RegimeCounter)
        {
          index = 0;
          num2 += 1;
        }
        if (num2 > 1)
          num1 = 0;
        if (!this.game.Data.RegimeObj[index].Sleep)
        {
          flag1 = this.game.Data.RegimeObj[index].AI;
          num1 = 0;
        }
      }
      this.game.EditObj.LastHistoryStep = -1;
      if (this.HumanPlayer > -1 & (flag1 | this.Curstep >= this.EndStep) && !this.game.AIRunning & !this.game.AIThreadRunning)
      {
        this.game.EditObj.AIMoving = false;
        this.game.EditObj.LastHistoryStep = this.Curstep;
        if (this.game.Data.UseAI == 0)
          this.game.AIObj.CloseAI();
        if (this.game.Data.UseAI == 1)
          this.game.NewAIObj.CloseAI();
        windowReturnClass.SetFlag(true);
        windowReturnClass.AddCommand(3, 4);
        return windowReturnClass;
      }
      bool flag2;
      if (this.AutoPlay & this.EndStep > 0)
      {
        TimeSpan timeSpan = DateAndTime.Now.Subtract(this.showtime);
        if ( timeSpan.Ticks > 1000000.0 & this.game.EditObj.HisLossCounter == -1 | timeSpan.Ticks > 20000000L)
        {
          if (this.HumanPlayer > -1)
            this.EndStep = this.game.HandyFunctionsObj.GetRegimeHistoryTotSteps(this.HumanPlayer);
          this.showtime = DateAndTime.Now;
          if (this.Curstep < this.EndStep | this.EndStep == -1 | this.game.EditObj.AIMoving & !this.game.AIRunning & !this.game.AIThreadRunning & this.Curstep >= this.EndStep)
          {
            if (!this.writing)
            {
              this.game.EditObj.TempCoordList = CoordList::new();
              this.Forward(1);
              this += 1.Curstep;
              this.StartStep = 1;
              if (this.Curstep == this.EndStep & this.HumanPlayer == -1)
                this.AutoPlay = false;
              if (this.HumanPlayer > -1 & (this.Curstep >= this.EndStep | this.EndStep == -1) && !this.game.AIRunning & !this.game.AIThreadRunning)
              {
                this.game.EditObj.AIMoving = false;
                if (this.game.Data.UseAI == 0)
                  this.game.AIObj.CloseAI();
                if (this.game.Data.UseAI == 1)
                  this.game.NewAIObj.CloseAI();
                windowReturnClass.SetFlag(true);
                windowReturnClass.AddCommand(3, 4);
                return windowReturnClass;
              }
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
              this.dostuff();
              flag2 = false;
            }
          }
          else if (this.game.AIRunning)
          {
            windowReturnClass.SetFlag(true);
            this.dostuff();
            flag2 = false;
          }
        }
        else if ( timeSpan.Ticks > 2500000.0 & this.game.AIRunning)
        {
          windowReturnClass.SetFlag(true);
          this.dostuff();
          flag2 = false;
        }
      }
      else
      {
        TimeSpan timeSpan = DateAndTime.Now.Subtract(this.showtime);
        if ( timeSpan.Ticks > 2500000.0 & this.game.AIRunning)
        {
          this.EndStep = this.game.HandyFunctionsObj.GetRegimeHistoryTotSteps(this.HumanPlayer);
          windowReturnClass.SetFlag(true);
          this.showtime = DateAndTime.Now;
          if (this.EndStep == this.lastendstep)
            ;
          flag2 = true;
        }
        else if (timeSpan.Ticks > 10000000L & !this.game.AIRunning & (this.HumanPlayer == -1 | !this.game.AIThreadRunning))
        {
          windowReturnClass.SetFlag(true);
          this.showtime = DateAndTime.Now;
          if (this.EndStep == this.lastendstep)
            ;
          flag2 = true;
        }
      }
      this.lastendstep = this.EndStep;
      if (!this.AutoPlay & !this.game.AIRunning & flag2)
        flag2 = false;
      if (flag2)
      {
        windowReturnClass.Flag = true;
        this.dostuff();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
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
            if (num1 == this.mapid)
            {
              let mut num2: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              if (num2 > -1)
              {
                this.detailnr = num2;
                this.game.EditObj.MapSelected = num2;
                this.game.CornerX = 0;
                this.game.CornerY = 0;
                this.game.SelectX = 0;
                this.game.SelectY = 0;
                this.game.EditObj.UnitSelected = -1;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 200, 150, false);
                windowReturnClass.AddCommand(4, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              return windowReturnClass;
            }
            num3: i32;
            if (num1 == this.SkipId)
            {
              if (this.Curstep == 0 & this.Curstep < this.EndStep)
              {
                this.Forward(1);
                this += 1.Curstep;
              }
              if (this.lastregime > -1)
              {
                let mut lastregime1: i32 =  this.lastregime;
                let mut num4: i32 =  1;
                while (num4 == 1)
                {
                  num4 = 0;
                  if (this.Curstep < this.EndStep)
                  {
                    this.Forward(1);
                    this += 1.Curstep;
                    num4 = 1;
                    let mut lastregime2: i32 =  this.lastregime;
                    if (lastregime2 != lastregime1 & lastregime2 != -1)
                    {
                      let mut num5: i32 =   Interaction.MsgBox( ("History from " + this.game.Data.RegimeObj[lastregime2].Name), Title: ( "Shadow Empire : Planetary Conquest"));
                      num4 = 0;
                    }
                  }
                }
                windowReturnClass.AddCommand(4, 12);
                this.dostuff();
                this.SubPartFlag[index] = true;
                windowReturnClass.SetFlag(true);
              }
            }
            else if (num1 == this.Slider1Id)
            {
              let mut steps: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index], b);
              if (steps > this.Curstep)
              {
                this.game.EditObj.TempCoordList = CoordList::new();
                this.Forward(steps - this.Curstep);
                windowReturnClass.AddCommand(4, 12);
                this.Curstep = steps;
                this.dostuff();
              }
              else if (steps < this.Curstep)
              {
                this.StartSit();
                this.Forward(steps);
                this.game.EditObj.TempCoordList = CoordList::new();
                windowReturnClass.AddCommand(4, 12);
                this.Curstep = steps;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.Info1Id)
            {
              this.StartSit();
              this.game.EditObj.TempCoordList = CoordList::new();
              this.Curstep = 0;
              windowReturnClass.AddCommand(4, 12);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.SpecialId)
            {
              if (this.Curstep < this.EndStep)
              {
                do
                {
                  this.Forward(1);
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this += 1.Curstep;
                }
                while (this.game.EditObj.HisAttackType == -1 & this.Curstep < this.EndStep);
              }
              windowReturnClass.AddCommand(4, 12);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.ExitId)
            {
              if (this.HumanPlayer > -1)
              {
                if (this.game.Data.UseAI == 0)
                  this.game.AIObj.CloseAI();
                if (this.game.Data.UseAI == 1)
                  this.game.NewAIObj.CloseAI();
                windowReturnClass.SetFlag(true);
                this.game.EditObj.AIMoving = false;
                windowReturnClass.AddCommand(3, 4);
                this.game.EditObj.OrderType = 0;
                this.game.EditObj.TempCoordList = CoordList::new();
                return windowReturnClass;
              }
              this.game.EditObj.TempCoordList = CoordList::new();
              if ( this.game.Data.RuleVar[839] == 0.0)
                windowReturnClass.AddCommand(3, 3);
              else
                windowReturnClass.AddCommand(3, 11);
              this.game.EditObj.OrderType = 0;
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.ViewAntiCapId)
            {
              this.game.EditObj.OrderType = 27;
              windowReturnClass.AddCommand(4, 12);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.ViewAntiCap2Id)
            {
              this.game.EditObj.OrderType = 28;
              windowReturnClass.AddCommand(4, 12);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.ViewAntiCap3Id)
            {
              this.game.EditObj.OrderType = 29;
              windowReturnClass.AddCommand(4, 12);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.ViewSupplyId)
            {
              this.game.EditObj.OrderType = 38;
              this.StartSit();
              windowReturnClass.AddCommand(4, 12);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.ViewHistoryId)
            {
              this.game.EditObj.OrderType = 26;
              windowReturnClass.AddCommand(4, 12);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.OptionsListId)
            {
              num3 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.OptionsList2Id)
            {
              num3 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.AutoPlayID)
            {
              this.AutoPlay = !this.AutoPlay;
              if (this.AutoPlay & this.Curstep >= this.EndStep)
              {
                this.StartSit();
                this.Curstep = 0;
              }
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.MiniMapId)
            {
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.dostuff();
              windowReturnClass.AddCommand(4, 12);
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
