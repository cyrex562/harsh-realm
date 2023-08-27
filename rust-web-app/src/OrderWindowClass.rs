// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.OrderWindowClass
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
  pub class OrderWindowClass : WindowClass
  {
     Info1Id: i32;
     Cancelid: i32;
     OkId: i32;
     BattleId: i32;
     LeftId: i32;
     RightId: i32;
     KillId: i32;
     AllId: i32;
     NoneId: i32;
     Ok2Id: i32;
     Battle2Id: i32;
     Kill2Id: i32;
     All2Id: i32;
     None2Id: i32;
     NotOkText: String;
     lastorderx: i32;
     lastordery: i32;
     bool TimerUsed;
     MoveButtonId: i32;
     StatisticsButtonId: i32;
     GroupMoveButtonId: i32;
     NextButtonId: i32;
     PopupButtonId: i32;
     NewUnitButtonId: i32;
     HqUnitButtonId: i32;
     NewUnitButton2Id: i32;
     AttackButtonId: i32;
     seaAttackButtonId: i32;
     PrefsButtonId: i32;
     ArtAttackButtonId: i32;
     SeaArtAttackButtonId: i32;
     TransferButtonId: i32;
     AirAttackButtonId: i32;
     InterdictButtonId: i32;
     StrategicButtonId: i32;
     GroupStrategicButtonId: i32;
     MakeHQButtonID: i32;
     AirReconButtonId: i32;
     ParadropButtonId: i32;
     LoadButtonId: i32;
     UnLoadButtonID: i32;
     ResearchId: i32;
     DipId: i32;
     HistoryId: i32;
     SaveId: i32;
     QuitID: i32;
     HqProdButtonId: i32;
     PeopleTransferButtonId: i32;
     ProdButtonId: i32;
     RecruitButtonId: i32;
     SupplyLayerButtonId: i32;
     AirSupplyButtonId: i32;
     OfficerId: i32;
     BlowBridgeButtonId: i32;
     BlowLocationButtonId: i32;
     ACapButtonId: i32;
     InfraButtonId: i32;
     BuildButtonId: i32;
     HexUnitButtonId: i32;
     HexUnitButtonId2: i32;
     GiveUnitId: i32;
     GiveHexId: i32;
     FakeBackButtonId: i32;
     ChangeModelId: i32;
     ChangeModelId2: i32;
     ModelDesignerId: i32;
     ModelDesignerId2: i32;
     SFDesignButtonId: i32;
     MoveButtonId2: i32;
     GroupMoveButtonId2: i32;
     StatisticsButtonId2: i32;
     NextButtonId2: i32;
     NewUnitButtonId2: i32;
     HqUnitButtonId2: i32;
     AttackButtonId2: i32;
     seaAttackButtonId2: i32;
     PrefsButtonId2: i32;
     NewUnitButton2Id2: i32;
     ArtAttackButtonId2: i32;
     SeaArtAttackButtonId2: i32;
     TransferButtonId2: i32;
     AirAttackButtonId2: i32;
     InterdictButtonId2: i32;
     StrategicButtonId2: i32;
     GroupStrategicButtonId2: i32;
     MakeHQButtonID2: i32;
     AirReconButtonId2: i32;
     ParadropButtonId2: i32;
     LoadButtonId2: i32;
     UnLoadButtonID2: i32;
     ResearchId2: i32;
     DipId2: i32;
     HistoryId2: i32;
     SaveId2: i32;
     QuitID2: i32;
     HqProdButtonId2: i32;
     OfficerId2: i32;
     PeopleTransferButtonId2: i32;
     ProdButtonId2: i32;
     RecruitButtonId2: i32;
     SupplyLayerButtonId2: i32;
     AirSupplyButtonId2: i32;
     OrderSurrenderButtonId: i32;
     ButtonZoomInId: i32;
     ButtonZoomOutId: i32;
     ButtonStackedUnitId: i32;
     OrderSurrenderButtonId2: i32;
     ButtonZoomInId2: i32;
     ButtonZoomOutId2: i32;
     ButtonStackedUnitId2: i32;
     BlowBridgeButtonId2: i32;
     BlowLocationButtonId2: i32;
     GiveUnitId2: i32;
     GiveHexId2: i32;
     ACapButtonId2: i32;
     InfraButtonId2: i32;
     BuildButtonId2: i32;
     SFDesignButtonText: String;
     MoveButtonText: String;
     ButtonZoomInText: String;
     ButtonZoomOutText: String;
     ButtonStackedUnitText: String;
     GroupMoveButtonText: String;
     StatisticsButtonText: String;
     NextButtonText: String;
     GiveUnitText: String;
     GiveHexText: String;
     NewUnitButtonText: String;
     HqUnitButtonText: String;
     AttackButtonText: String;
     ChangeModelText: String;
     OfficerText: String;
     SeaAttackButtonText: String;
     PrefsButtonText: String;
     ArtAttackButtonText: String;
     SeaArtAttackButtonText: String;
     TransferButtonText: String;
     AirAttackButtonText: String;
     AirReconButtonText: String;
     newunitbutton2text: String;
     paradropbuttontext: String;
     loadbuttontext: String;
     unloadbuttontext: String;
     researchbuttontext: String;
     diptext: String;
     constructtext: String;
     historytext: String;
     savetext: String;
     quittext: String;
     hqprodbuttontext: String;
     ordersurrendertext: String;
     supplylayerbuttontext: String;
     blowlocationtext: String;
     disbandtext: String;
     interdictbuttontext: String;
     prodbuttontext: String;
     researchtext: String;
     groupstrategictext: String;
     strategicbuttontext: String;
     airsupplybuttontext: String;
     blowbridgebuttontext: String;
     infrabuttontext: String;
     buildbuttontext: String;
     disbandid: i32;
     disbandid2: i32;

    pub OrderWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, 40, 8, tBackSpriteScaled: true, tTransBacksprite: true)
    {
      this.lastorderx = -1;
      this.lastordery = -1;
      this.dostuff();
    }

    pub fn DoRefresh() => this.dostuff();

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (!this.TimerUsed && this.game.EditObj.OrderType == 18 | this.game.EditObj.OrderType == 49 && this.game.EditObj.OrderTarget > -1)
      {
        this.game.EditObj.TempCoordList = CoordList::new();
        this.game.EditObj.TargetX = this.game.EditObj.OrderX;
        this.game.EditObj.TargetY = this.game.EditObj.OrderY;
        this.game.SelectX = this.game.EditObj.OrderX;
        this.game.SelectY = this.game.EditObj.OrderY;
        this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectX, this.game.EditObj.MapSelected);
        windowReturnClass.AddCommand(1, 5);
        windowReturnClass.AddCommand(4, 18);
        windowReturnClass.AddCommand(2, 35);
        windowReturnClass.AddCommand(4, 12);
        this.TimerUsed = true;
        windowReturnClass.SetFlag(true);
      }
      return windowReturnClass;
    }

    pub fn dostuff()
    {
      SizeF sizeF1 = SizeF::new();
      if (this.Info1Id > 0)
        this.RemoveSubPart(this.Info1Id);
      if (this.Cancelid > 0)
      {
        this.RemoveSubPart(this.Cancelid);
        this.Cancelid = 0;
      }
      if (this.OkId > 0)
      {
        this.RemoveSubPart(this.OkId);
        this.OkId = 0;
      }
      if (this.KillId > 0)
      {
        this.RemoveSubPart(this.KillId);
        this.KillId = 0;
      }
      if (this.Ok2Id > 0)
      {
        this.RemoveSubPart(this.Ok2Id);
        this.Ok2Id = 0;
      }
      if (this.Kill2Id > 0)
      {
        this.RemoveSubPart(this.Kill2Id);
        this.Kill2Id = 0;
      }
      if (this.PopupButtonId > 0)
      {
        this.RemoveSubPart(this.PopupButtonId);
        this.PopupButtonId = 0;
      }
      if (this.SupplyLayerButtonId > 0)
      {
        this.RemoveSubPart(this.SupplyLayerButtonId);
        this.SupplyLayerButtonId = 0;
      }
      if (this.BlowBridgeButtonId > 0)
      {
        this.RemoveSubPart(this.BlowBridgeButtonId);
        this.BlowBridgeButtonId = 0;
      }
      if (this.InfraButtonId > 0)
      {
        this.RemoveSubPart(this.InfraButtonId);
        this.InfraButtonId = 0;
      }
      if (this.BuildButtonId > 0)
      {
        this.RemoveSubPart(this.BuildButtonId);
        this.BuildButtonId = 0;
      }
      if (this.AirReconButtonId > 0)
      {
        this.RemoveSubPart(this.AirReconButtonId);
        this.AirReconButtonId = 0;
      }
      if (this.MoveButtonId > 0)
      {
        this.RemoveSubPart(this.MoveButtonId);
        this.MoveButtonId = 0;
      }
      if (this.GroupMoveButtonId > 0)
      {
        this.RemoveSubPart(this.GroupMoveButtonId);
        this.GroupMoveButtonId = 0;
      }
      if (this.NextButtonId > 0)
      {
        this.RemoveSubPart(this.NextButtonId);
        this.NextButtonId = 0;
      }
      if (this.HqUnitButtonId > 0)
      {
        this.RemoveSubPart(this.HqUnitButtonId);
        this.HqUnitButtonId = 0;
      }
      if (this.NewUnitButtonId > 0)
      {
        this.RemoveSubPart(this.NewUnitButtonId);
        this.NewUnitButtonId = 0;
      }
      if (this.NewUnitButton2Id2 > 0)
      {
        this.RemoveSubPart(this.NewUnitButton2Id2);
        this.NewUnitButton2Id2 = 0;
      }
      if (this.NewUnitButton2Id > 0)
      {
        this.RemoveSubPart(this.NewUnitButton2Id);
        this.NewUnitButton2Id = 0;
      }
      if (this.TransferButtonId > 0)
      {
        this.RemoveSubPart(this.TransferButtonId);
        this.TransferButtonId = 0;
      }
      if (this.AttackButtonId > 0)
      {
        this.RemoveSubPart(this.AttackButtonId);
        this.AttackButtonId = 0;
      }
      if (this.seaAttackButtonId > 0)
      {
        this.RemoveSubPart(this.seaAttackButtonId);
        this.seaAttackButtonId = 0;
      }
      if (this.ArtAttackButtonId > 0)
      {
        this.RemoveSubPart(this.ArtAttackButtonId);
        this.ArtAttackButtonId = 0;
      }
      if (this.SeaArtAttackButtonId > 0)
      {
        this.RemoveSubPart(this.SeaArtAttackButtonId);
        this.SeaArtAttackButtonId = 0;
      }
      if (this.AirAttackButtonId > 0)
      {
        this.RemoveSubPart(this.AirAttackButtonId);
        this.AirAttackButtonId = 0;
      }
      if (this.InterdictButtonId > 0)
      {
        this.RemoveSubPart(this.InterdictButtonId);
        this.InterdictButtonId = 0;
      }
      if (this.StrategicButtonId > 0)
      {
        this.RemoveSubPart(this.StrategicButtonId);
        this.StrategicButtonId = 0;
      }
      if (this.GroupStrategicButtonId > 0)
      {
        this.RemoveSubPart(this.GroupStrategicButtonId);
        this.GroupStrategicButtonId = 0;
      }
      if (this.ParadropButtonId > 0)
      {
        this.RemoveSubPart(this.ParadropButtonId);
        this.ParadropButtonId = 0;
      }
      if (this.LoadButtonId > 0)
      {
        this.RemoveSubPart(this.LoadButtonId);
        this.LoadButtonId = 0;
      }
      if (this.UnLoadButtonID > 0)
      {
        this.RemoveSubPart(this.UnLoadButtonID);
        this.UnLoadButtonID = 0;
      }
      if (this.ResearchId > 0)
      {
        this.RemoveSubPart(this.ResearchId);
        this.ResearchId = 0;
      }
      if (this.DipId > 0)
      {
        this.RemoveSubPart(this.DipId);
        this.DipId = 0;
      }
      if (this.HistoryId > 0)
      {
        this.RemoveSubPart(this.HistoryId);
        this.HistoryId = 0;
      }
      if (this.SaveId > 0)
      {
        this.RemoveSubPart(this.SaveId);
        this.SaveId = 0;
      }
      if (this.QuitID > 0)
      {
        this.RemoveSubPart(this.QuitID);
        this.QuitID = 0;
      }
      if (this.HqProdButtonId > 0)
      {
        this.RemoveSubPart(this.HqProdButtonId);
        this.HqProdButtonId = 0;
      }
      if (this.ProdButtonId > 0)
      {
        this.RemoveSubPart(this.ProdButtonId);
        this.ProdButtonId = 0;
      }
      if (this.disbandid > 0)
      {
        this.RemoveSubPart(this.disbandid);
        this.disbandid = 0;
      }
      if (this.PrefsButtonId > 0)
      {
        this.RemoveSubPart(this.PrefsButtonId);
        this.PrefsButtonId = 0;
      }
      if (this.AirSupplyButtonId > 0)
      {
        this.RemoveSubPart(this.AirSupplyButtonId);
        this.AirSupplyButtonId = 0;
      }
      if (this.StatisticsButtonId > 0)
      {
        this.RemoveSubPart(this.StatisticsButtonId);
        this.StatisticsButtonId = 0;
      }
      if (this.BlowLocationButtonId > 0)
      {
        this.RemoveSubPart(this.BlowLocationButtonId);
        this.BlowLocationButtonId = 0;
      }
      if (this.OrderSurrenderButtonId > 0)
      {
        this.RemoveSubPart(this.OrderSurrenderButtonId);
        this.OrderSurrenderButtonId = 0;
      }
      if (this.HexUnitButtonId2 > 0)
      {
        this.RemoveSubPart(this.HexUnitButtonId2);
        this.HexUnitButtonId2 = 0;
      }
      if (this.HexUnitButtonId > 0)
      {
        this.RemoveSubPart(this.HexUnitButtonId);
        this.HexUnitButtonId = 0;
      }
      if (this.NewUnitButton2Id > 0)
      {
        this.RemoveSubPart(this.NewUnitButton2Id);
        this.NewUnitButton2Id = 0;
      }
      if (this.GiveUnitId > 0)
      {
        this.RemoveSubPart(this.GiveUnitId);
        this.GiveUnitId = 0;
      }
      if (this.GiveHexId > 0)
      {
        this.RemoveSubPart(this.GiveHexId);
        this.GiveHexId = 0;
      }
      if (this.SFDesignButtonId > 0)
      {
        this.RemoveSubPart(this.SFDesignButtonId);
        this.SFDesignButtonId = 0;
      }
      if (this.OfficerId > 0)
      {
        this.RemoveSubPart(this.OfficerId);
        this.OfficerId = 0;
      }
      if (this.OfficerId2 > 0)
      {
        this.RemoveSubPart(this.OfficerId2);
        this.OfficerId2 = 0;
      }
      if (this.ButtonZoomInId > 0)
      {
        this.RemoveSubPart(this.ButtonZoomInId);
        this.ButtonZoomInId = 0;
      }
      if (this.ButtonZoomInId2 > 0)
      {
        this.RemoveSubPart(this.ButtonZoomInId2);
        this.ButtonZoomInId2 = 0;
      }
      if (this.ButtonZoomOutId > 0)
      {
        this.RemoveSubPart(this.ButtonZoomOutId);
        this.ButtonZoomOutId = 0;
      }
      if (this.ButtonZoomOutId2 > 0)
      {
        this.RemoveSubPart(this.ButtonZoomOutId2);
        this.ButtonZoomOutId2 = 0;
      }
      if (this.ButtonStackedUnitId > 0)
      {
        this.RemoveSubPart(this.ButtonStackedUnitId);
        this.ButtonStackedUnitId = 0;
      }
      if (this.ButtonStackedUnitId2 > 0)
      {
        this.RemoveSubPart(this.ButtonStackedUnitId2);
        this.ButtonStackedUnitId2 = 0;
      }
      if (this.SupplyLayerButtonId2 > 0)
      {
        this.RemoveSubPart(this.SupplyLayerButtonId2);
        this.SupplyLayerButtonId2 = 0;
      }
      if (this.BlowBridgeButtonId2 > 0)
      {
        this.RemoveSubPart(this.BlowBridgeButtonId2);
        this.BlowBridgeButtonId2 = 0;
      }
      if (this.InfraButtonId2 > 0)
      {
        this.RemoveSubPart(this.InfraButtonId2);
        this.InfraButtonId2 = 0;
      }
      if (this.BuildButtonId2 > 0)
      {
        this.RemoveSubPart(this.BuildButtonId2);
        this.BuildButtonId2 = 0;
      }
      if (this.AirReconButtonId2 > 0)
      {
        this.RemoveSubPart(this.AirReconButtonId2);
        this.AirReconButtonId2 = 0;
      }
      if (this.MoveButtonId2 > 0)
      {
        this.RemoveSubPart(this.MoveButtonId2);
        this.MoveButtonId2 = 0;
      }
      if (this.GroupMoveButtonId2 > 0)
      {
        this.RemoveSubPart(this.GroupMoveButtonId2);
        this.GroupMoveButtonId2 = 0;
      }
      if (this.NextButtonId2 > 0)
      {
        this.RemoveSubPart(this.NextButtonId2);
        this.NextButtonId2 = 0;
      }
      if (this.HqUnitButtonId2 > 0)
      {
        this.RemoveSubPart(this.HqUnitButtonId2);
        this.HqUnitButtonId2 = 0;
      }
      if (this.NewUnitButtonId2 > 0)
      {
        this.RemoveSubPart(this.NewUnitButtonId2);
        this.NewUnitButtonId2 = 0;
      }
      if (this.TransferButtonId2 > 0)
      {
        this.RemoveSubPart(this.TransferButtonId2);
        this.TransferButtonId2 = 0;
      }
      if (this.AttackButtonId2 > 0)
      {
        this.RemoveSubPart(this.AttackButtonId2);
        this.AttackButtonId2 = 0;
      }
      if (this.seaAttackButtonId2 > 0)
      {
        this.RemoveSubPart(this.seaAttackButtonId2);
        this.seaAttackButtonId2 = 0;
      }
      if (this.ArtAttackButtonId2 > 0)
      {
        this.RemoveSubPart(this.ArtAttackButtonId2);
        this.ArtAttackButtonId2 = 0;
      }
      if (this.SeaArtAttackButtonId2 > 0)
      {
        this.RemoveSubPart(this.SeaArtAttackButtonId2);
        this.SeaArtAttackButtonId2 = 0;
      }
      if (this.AirAttackButtonId2 > 0)
      {
        this.RemoveSubPart(this.AirAttackButtonId2);
        this.AirAttackButtonId2 = 0;
      }
      if (this.InterdictButtonId2 > 0)
      {
        this.RemoveSubPart(this.InterdictButtonId2);
        this.InterdictButtonId2 = 0;
      }
      if (this.StrategicButtonId2 > 0)
      {
        this.RemoveSubPart(this.StrategicButtonId2);
        this.StrategicButtonId2 = 0;
      }
      if (this.GroupStrategicButtonId2 > 0)
      {
        this.RemoveSubPart(this.GroupStrategicButtonId2);
        this.GroupStrategicButtonId2 = 0;
      }
      if (this.ParadropButtonId2 > 0)
      {
        this.RemoveSubPart(this.ParadropButtonId2);
        this.ParadropButtonId2 = 0;
      }
      if (this.LoadButtonId2 > 0)
      {
        this.RemoveSubPart(this.LoadButtonId2);
        this.LoadButtonId2 = 0;
      }
      if (this.UnLoadButtonID2 > 0)
      {
        this.RemoveSubPart(this.UnLoadButtonID2);
        this.UnLoadButtonID2 = 0;
      }
      if (this.ResearchId2 > 0)
      {
        this.RemoveSubPart(this.ResearchId2);
        this.ResearchId2 = 0;
      }
      if (this.DipId2 > 0)
      {
        this.RemoveSubPart(this.DipId2);
        this.DipId2 = 0;
      }
      if (this.HistoryId2 > 0)
      {
        this.RemoveSubPart(this.HistoryId2);
        this.HistoryId2 = 0;
      }
      if (this.SaveId2 > 0)
      {
        this.RemoveSubPart(this.SaveId2);
        this.SaveId2 = 0;
      }
      if (this.QuitID2 > 0)
      {
        this.RemoveSubPart(this.QuitID2);
        this.QuitID2 = 0;
      }
      if (this.HqProdButtonId2 > 0)
      {
        this.RemoveSubPart(this.HqProdButtonId2);
        this.HqProdButtonId2 = 0;
      }
      if (this.ProdButtonId2 > 0)
      {
        this.RemoveSubPart(this.ProdButtonId2);
        this.ProdButtonId2 = 0;
      }
      if (this.disbandid2 > 0)
      {
        this.RemoveSubPart(this.disbandid2);
        this.disbandid2 = 0;
      }
      if (this.PrefsButtonId2 > 0)
      {
        this.RemoveSubPart(this.PrefsButtonId2);
        this.PrefsButtonId2 = 0;
      }
      if (this.AirSupplyButtonId2 > 0)
      {
        this.RemoveSubPart(this.AirSupplyButtonId2);
        this.AirSupplyButtonId2 = 0;
      }
      if (this.StatisticsButtonId2 > 0)
      {
        this.RemoveSubPart(this.StatisticsButtonId2);
        this.StatisticsButtonId2 = 0;
      }
      if (this.BlowLocationButtonId2 > 0)
      {
        this.RemoveSubPart(this.BlowLocationButtonId2);
        this.BlowLocationButtonId2 = 0;
      }
      if (this.OrderSurrenderButtonId2 > 0)
      {
        this.RemoveSubPart(this.OrderSurrenderButtonId2);
        this.OrderSurrenderButtonId2 = 0;
      }
      if (this.GiveUnitId2 > 0)
      {
        this.RemoveSubPart(this.GiveUnitId2);
        this.GiveUnitId2 = 0;
      }
      if (this.GiveHexId2 > 0)
      {
        this.RemoveSubPart(this.GiveHexId2);
        this.GiveHexId2 = 0;
      }
      if (this.ChangeModelId > 0)
      {
        this.RemoveSubPart(this.ChangeModelId);
        this.ChangeModelId = 0;
      }
      if (this.ChangeModelId2 > 0)
      {
        this.RemoveSubPart(this.ChangeModelId2);
        this.ChangeModelId = 0;
      }
      if (this.ModelDesignerId > 0)
      {
        this.RemoveSubPart(this.ModelDesignerId);
        this.ChangeModelId = 0;
      }
      if (this.FakeBackButtonId > 0)
      {
        this.RemoveSubPart(this.FakeBackButtonId);
        this.FakeBackButtonId = 0;
      }
      if (this.BattleId > 0)
      {
        this.RemoveSubPart(this.BattleId);
        this.BattleId = 0;
      }
      if (this.LeftId > 0)
      {
        this.RemoveSubPart(this.LeftId);
        this.LeftId = 0;
      }
      if (this.RightId > 0)
      {
        this.RemoveSubPart(this.RightId);
        this.RightId = 0;
      }
      if (this.AllId > 0)
      {
        this.RemoveSubPart(this.AllId);
        this.AllId = 0;
      }
      if (this.NoneId > 0)
      {
        this.RemoveSubPart(this.NoneId);
        this.NoneId = 0;
      }
      if (this.BattleId > 0)
      {
        this.RemoveSubPart(this.BattleId);
        this.BattleId = 0;
      }
      if (this.All2Id > 0)
      {
        this.RemoveSubPart(this.All2Id);
        this.All2Id = 0;
      }
      if (this.None2Id > 0)
      {
        this.RemoveSubPart(this.None2Id);
        this.None2Id = 0;
      }
      if (this.Battle2Id > 0)
      {
        this.RemoveSubPart(this.Battle2Id);
        this.Battle2Id = 0;
      }
      this.NewBackGroundAndClearAll(this.game.ScreenWidth, 40, -1);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawBlock( Expression, 0, 0, this.game.ScreenWidth, 40,  this.game.VicColor5.R,  this.game.VicColor5.G,  this.game.VicColor5.B,  this.game.VicColor5.A);
      this.OfficerText = "You have to select an HQ in order to access the officerpool window.";
      if (this.game.EditObj.OrderType == 0)
      {
        this.PrefsButtonId = 1;
        if (this.game.Data.Round > 0)
        {
          this.ResearchId = 1;
          this.SFDesignButtonId = 1;
          if (this.game.Data.Turn > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
            this.ModelDesignerId = 1;
          this.DipId = 1;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].HistoryStepCounter > -1)
            this.HistoryId = 1;
          else
            this.historytext = "No history records to report!";
          this.NextButtonId = 1;
          if (this.game.Data.Winner == -1)
            this.OrderSurrenderButtonId = 1;
          else
            this.ordersurrendertext = "Somebody already won the game.";
          this.StatisticsButtonId = 1;
        }
        if (this.game.SelectX > -1 & this.game.SelectY > -1)
        {
          this.HexUnitButtonId = 1;
          this.ButtonZoomInId = 1;
          this.ButtonZoomOutId = 1;
          this.ButtonStackedUnitId = 1;
          if (this.game.SelectX <= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth && this.game.SelectY <= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight && this.game.Data.Round > 0)
          {
            if (!this.game.Data.ShrowdOn)
              this.SupplyLayerButtonId = 1;
            else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn)
              this.SupplyLayerButtonId = 1;
          }
        }
        if (this.game.SelectX > -1 && this.game.Data.Round == 0 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1)
          this.NewUnitButtonId = 1;
        if (this.game.EditObj.UnitSelected > -1)
        {
          if (this.game.Data.Turn > -1 & this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime == this.game.Data.Turn)
          {
            if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
            {
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ & this.game.Data.RegimeObj[this.game.Data.Turn].OfficerPool > -1 & this.game.Data.Round > 0)
                this.OfficerId = 1;
              else
                this.OfficerText = "Officerpool not available";
            }
            else
              this.ChangeModelId = 1;
          }
          if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn | this.game.Data.Round == 0)
          {
            if (this.game.Data.Turn > -1 &&  this.game.Data.RuleVar[528] == 1.0 && this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
            {
              if (this.game.HandyFunctionsObj.HasAllies(this.game.Data.Turn, true))
                this.GiveUnitId = 1;
              else
                this.GiveUnitText = "You have no allies.";
            }
            if (this.game.Data.Round == 0)
              this.MoveButtonId = 1;
            else if (this.game.HandyFunctionsObj.CanUnitMove(this.game.EditObj.UnitSelected))
            {
              let mut airCarryCapPts: i32 =  this.game.HandyFunctionsObj.GetAirCarryCapPts(this.game.EditObj.UnitSelected);
              if (Conversions.ToInteger(Operators.SubtractObject(this.game.HandyFunctionsObj.GetUnitNonSeaWeight(this.game.EditObj.UnitSelected, true), this.game.HandyFunctionsObj.GetUnitNonSeaWeight(this.game.EditObj.UnitSelected, false))) > airCarryCapPts & airCarryCapPts > 0)
              {
                this.MoveButtonId = 0;
                this.MoveButtonText = "Unit does not have enough aircraftcarrier capacity.";
                this.GroupMoveButtonId = 0;
                this.GroupMoveButtonText = "Unit does not have enough aircraftcarrier capacity.";
              }
              else
              {
                this.MoveButtonId = 1;
                if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].StartSize > 1)
                  this.GroupMoveButtonId = 1;
              }
            }
            else
            {
              this.MoveButtonId = 0;
              this.MoveButtonText = "Unit does not have enough Action Points to move.";
              this.GroupMoveButtonId = 0;
              this.GroupMoveButtonText = "Unit does not have enough Action Points to move.";
            }
            this.HqUnitButtonId = 1;
            if (this.game.Data.Turn > -1 && this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.UnitSelected, false) > this.game.Data.RegimeObj[this.game.Data.Turn].ResPts)
            {
              this.HqUnitButtonId = 0;
              this.HqUnitButtonText = "You dont have the required " + Conversion.Str( this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.UnitSelected, false)) + " PP to switch the HQ of this unit.";
            }
            if (this.game.Data.Turn > -1 &&  this.game.Data.RuleVar[343] == 1.0 & this.game.Data.Turn > -1 & this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime <= -1)
            {
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
              {
                if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ & this.game.Data.RegimeObj[this.game.Data.Turn].OfficerPool > -1 & this.game.Data.Round > 0)
                  this.OfficerId = 1;
                else
                  this.OfficerText = "Officerpool not available";
              }
              else
                this.OfficerText = "Only HQs can have officers";
            }
            if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
              this.ChangeModelId = 1;
            else
              this.ChangeModelText = "You cannot change Model of a HQ unit.";
            if (this.game.Data.Round > 0)
            {
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount == -1)
              {
                this.disbandid = 1;
                if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].ResPts + this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].PP < 0)
                {
                  this.disbandid = 0;
                  this.disbandtext = "You do not have the PP to disband the unit.";
                }
              }
              else
              {
                this.disbandid = 0;
                this.disbandtext = "You can only disband a unit with no subformations in it.";
              }
            }
            if (this.game.Data.Round > 0)
            {
              let mut num: i32 =  0;
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 && this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].StructuralPts < this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].StructuralPts &&  this.game.Data.RuleVar[902] < 1.0 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].EPCost > 0)
                num = 1;
              if (this.game.HandyFunctionsObj.CanUnitBuild(this.game.EditObj.UnitSelected) | num == 1)
              {
                if (this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) > 0)
                {
                  if ( this.game.Data.RuleVar[868] > 0.0 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_APPenalty(this.game.Data.Turn) > 0)
                    this.buildbuttontext = "You cannot build/repair something if you conquered the hex in the current turn. Please wait a full turn.";
                  else
                    this.BuildButtonId = 1;
                }
                else
                  this.buildbuttontext = "You cannot build/repair something if you have zero action points.";
              }
              else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1)
              {
                if ( this.game.Data.RuleVar[902] < 1.0)
                {
                  this.BuildButtonId = 0;
                  this.buildbuttontext = "Location is not damaged / Unit cannot repair this location";
                }
                else
                {
                  this.BuildButtonId = 0;
                  this.buildbuttontext = "Location cannot be repaired.";
                }
              }
              else
              {
                this.BuildButtonId = 0;
                this.buildbuttontext = "Building not an option.";
              }
            }
            if (this.game.Data.Round > 0)
            {
              if (this.game.HandyFunctionsObj.GetUnitEP(this.game.EditObj.UnitSelected) > 0)
              {
                if (this.game.HandyFunctionsObj.CanUnitInfra(this.game.EditObj.UnitSelected))
                {
                  if (this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) > 0)
                  {
                    this.InfraButtonId = 1;
                  }
                  else
                  {
                    this.InfraButtonId = 0;
                    this.infrabuttontext = "Unit has 0 action points.";
                  }
                }
                else
                {
                  this.InfraButtonId = 0;
                  this.infrabuttontext = "Unit is not capable of build road.";
                }
              }
              else
              {
                this.InfraButtonId = 0;
                this.infrabuttontext = "Unit has no engineer points.";
              }
            }
            if (this.game.Data.Round > 0)
            {
              if (this.game.HandyFunctionsObj.CanWeBlowBridge(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected))
              {
                if (this.game.HandyFunctionsObj.GetBlowBridgePts(this.game.EditObj.UnitSelected) > 0)
                {
                  if (this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) > 49)
                  {
                    this.BlowBridgeButtonId = 1;
                  }
                  else
                  {
                    this.BlowBridgeButtonId = 0;
                    this.blowbridgebuttontext = "You need at least 50 Action Pts for an attempt.";
                  }
                }
                else
                {
                  this.BlowBridgeButtonId = 0;
                  this.blowbridgebuttontext = "Unit does not have capability to blow bridge.";
                }
              }
              else
              {
                this.BlowBridgeButtonId = 0;
                this.blowbridgebuttontext = "No bridge is present in this hex.";
              }
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1)
              {
                if (!this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].Invincible)
                {
                  if (this.game.HandyFunctionsObj.GetBlowBridgePts(this.game.EditObj.UnitSelected) > 0)
                  {
                    if (this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) > 49)
                    {
                      this.BlowLocationButtonId = 1;
                    }
                    else
                    {
                      this.blowlocationtext = "You need at least 50 Action Pts for an attempt.";
                      this.BlowLocationButtonId = 0;
                    }
                  }
                  else
                  {
                    this.blowlocationtext = "Unit does not have capability to raze location.";
                    this.BlowLocationButtonId = 0;
                  }
                }
                else
                {
                  this.blowlocationtext = "This locationtype cannot be damaged.";
                  this.BlowLocationButtonId = 0;
                }
              }
              else
              {
                this.blowlocationtext = "No Location in hex.";
                this.BlowLocationButtonId = 0;
              }
            }
            if (this.game.Data.Round > 0)
            {
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerCounter == -1)
              {
                if ( this.game.Data.RuleVar[315] == 1.0 & !this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
                {
                  this.TransferButtonId = 0;
                  this.TransferButtonText = "You can only transfer from a HQ.";
                }
                else
                  this.TransferButtonId = 1;
                if (!this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected) & !this.game.HandyFunctionsObj.HasUnitNavySF(this.game.EditObj.UnitSelected))
                {
                  if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y].get_APPenalty(this.game.Data.Turn) > 0)
                  {
                    this.strategicbuttontext = "You cant strategically transfer a unit from a hex you conquered in this turn.";
                    this.groupstrategictext = "You cant strategically transfer a unit from a hex you conquered in this turn.";
                  }
                  else if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].DidAttack)
                  {
                    this.StrategicButtonId = 1;
                    if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].StartSize > -1)
                    {
                      let mut num: i32 =  1;
                      let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
                      {
                        if (this.game.Data.UnitObj[index].Historical == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical & this.game.Data.UnitObj[index].Regime == this.game.Data.Turn && this.game.Data.UnitObj[index].DidAttack)
                          num = 0;
                      }
                      if (num == 1)
                        this.GroupStrategicButtonId = 1;
                    }
                  }
                  else
                  {
                    this.strategicbuttontext = "You cannot strategic transfer a unit that participated in an attack.";
                    this.groupstrategictext = "You cannot strategic transfer a unit that participated in an attack.";
                  }
                }
                else
                {
                  this.groupstrategictext = "You cannot strategic transfer units with air or navy subformations.";
                  this.strategicbuttontext = "You cannot strategic transfer units with air or navy subformations.";
                }
              }
              else
              {
                this.TransferButtonText = "You cannot transfer while carrying passengers.";
                this.strategicbuttontext = "You cannot strategicly transfer a unit carrying passengers.";
                this.groupstrategictext = "You cannot strategicly transfer a unit carrying passengers.";
              }
            }
            if (this.game.Data.Round > 0)
            {
              if (this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected))
              {
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1)
                {
                  if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].IsAirfield)
                  {
                    if (this.game.HandyFunctionsObj.GetUnitCarryCap(this.game.EditObj.UnitSelected, 2) > 0)
                      this.ParadropButtonId = 1;
                    else
                      this.paradropbuttontext = "Unit has no aircraft that can carry paratroopers.";
                    this.AirReconButtonId = 1;
                    if (this.game.HandyFunctionsObj.GetCarryCapPts(this.game.EditObj.UnitSelected, 2) > 0)
                      this.AirSupplyButtonId = 1;
                    else
                      this.airsupplybuttontext = "Units air subformations have no carry points.";
                  }
                  else
                  {
                    this.paradropbuttontext = "Action must start from airfield. No airfield in this hex.";
                    this.AirReconButtonText = "Action must start from airfield. No airfield in this hex.";
                    this.airsupplybuttontext = "Action must start from airfield. No airfield in this hex.";
                  }
                }
                else
                {
                  this.paradropbuttontext = "Action must start from airfield. No airfield in this hex.";
                  this.AirReconButtonText = "Action must start from airfield. No airfield in this hex.";
                  this.airsupplybuttontext = "Action must start from airfield. No airfield in this hex.";
                }
                if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea)
                  this.AirReconButtonId = 1;
              }
              else
              {
                this.paradropbuttontext = "Unit has no aircraft.";
                this.AirReconButtonText = "Unit has no aircraft.";
                this.airsupplybuttontext = "Unit has no aircraft";
              }
            }
            if (this.game.HandyFunctionsObj.HasUnitNavySF(this.game.EditObj.UnitSelected))
            {
              if (this.game.HandyFunctionsObj.GetUnitCarryCap(this.game.EditObj.UnitSelected, 1) > 0)
              {
                if (this.game.HandyFunctionsObj.CanUnitLoadaUnit(this.game.EditObj.UnitSelected))
                  this.LoadButtonId = 1;
                else
                  this.loadbuttontext = "No unit around to be loaded by this unit.";
              }
              else
                this.loadbuttontext = "Unit has no ships that can carry cargo.";
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerCounter > -1)
              {
                if (this.game.EditObj.SFSelected > this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount)
                {
                  if (this.game.HandyFunctionsObj.GetLowestAp(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerList[this.game.EditObj.SFSelected - (1 + this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount)]) > 0 | this.game.Data.Round == 0 | this.game.HandyFunctionsObj.IsHexPort(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Map))
                    this.UnLoadButtonID = 1;
                  else
                    this.unloadbuttontext = "Passenger must have more then 0 ap to unload.";
                }
                else
                  this.unloadbuttontext = "Unit has passengers, but no unit is selected to unload";
              }
              else
                this.unloadbuttontext = "Unit has no passenger(s).";
            }
            else
            {
              this.loadbuttontext = "Unit has no navy subformations.";
              this.unloadbuttontext = "Unit has no navy subformations.";
            }
          }
          else
          {
            this.HqUnitButtonText = "Not a friendly unit.";
            if (this.game.Data.Turn > -1 && this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime == this.game.Data.Turn)
            {
              this.HqUnitButtonId = 1;
              if (this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.UnitSelected, false) > this.game.Data.RegimeObj[this.game.Data.Turn].ResPts)
              {
                this.HqUnitButtonId = 0;
                this.HqUnitButtonText = "You dont have the required " + Conversion.Str( this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.UnitSelected, false)) + " PP to switch the HQ of this unit.";
              }
            }
            this.supplylayerbuttontext = "Not a friendly unit.";
            this.MoveButtonText = "Not a friendly unit.";
            this.GroupMoveButtonText = "Not a friendly unit.";
            this.disbandtext = "Not a friendly unit.";
            this.buildbuttontext = "Not a friendly unit.";
            this.infrabuttontext = "Not a friendly unit.";
            this.blowbridgebuttontext = "Not a friendly unit.";
            this.blowlocationtext = "Not a friendly unit.";
            this.TransferButtonText = "Not a friendly unit.";
            this.strategicbuttontext = "Not a friendly unit.";
            this.groupstrategictext = "Not a friendly unit.";
            this.paradropbuttontext = "Not a friendly unit.";
            this.AirReconButtonText = "Not a friendly unit.";
            this.airsupplybuttontext = "Not a friendly unit.";
            this.loadbuttontext = "Not a friendly unit.";
            this.unloadbuttontext = "Not a friendly unit.";
            this.GiveUnitText = "Not a friendly unit.";
            this.OfficerText = "Not a friendly unit.";
            this.ChangeModelText = "Not a friendly unit.";
          }
        }
        else
        {
          this.groupstrategictext = "No unit selected.";
          this.supplylayerbuttontext = "No unit selected.";
          this.MoveButtonText = "No unit selected.";
          this.GroupMoveButtonText = "No unit selected.";
          this.HqUnitButtonText = "No unit selected.";
          this.disbandtext = "No unit selected.";
          this.buildbuttontext = "No unit selected.";
          this.infrabuttontext = "No unit selected.";
          this.blowbridgebuttontext = "No unit selected.";
          this.blowlocationtext = "No unit selected.";
          this.TransferButtonText = "No unit selected.";
          this.strategicbuttontext = "No unit selected.";
          this.paradropbuttontext = "No unit selected.";
          this.AirReconButtonText = "No unit selected.";
          this.airsupplybuttontext = "No unit selected.";
          this.loadbuttontext = "No unit selected.";
          this.unloadbuttontext = "No unit selected.";
          this.GiveUnitText = "Not a friendly unit.";
          this.OfficerText = "Not a friendly unit.";
          this.ChangeModelText = "Not a friendly unit.";
          if (this.game.SelectX > -1 && this.game.Data.Round > 0 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location == -1)
          {
            let mut locTypeCounter: i32 =  this.game.Data.LocTypeCounter;
            for (let mut index: i32 =  0; index <= locTypeCounter; index += 1)
            {
              if (this.game.Data.LocTypeObj[index].Buildable && this.game.Data.LocTypeObj[index].HumanCanBuild)
                this.BuildButtonId = 1;
            }
          }
        }
        if (this.game.SelectX > -1 & this.game.SelectY > -1 && this.game.Data.Turn > -1 &&  this.game.Data.RuleVar[529] == 1.0)
        {
          if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn)
          {
            if (this.game.HandyFunctionsObj.HasAllies(this.game.Data.Turn, false))
              this.GiveHexId = 1;
            else
              this.GiveHexText = "You have no allies.";
          }
          else
            this.GiveHexText = !this.game.HandyFunctionsObj.HasAllies(this.game.Data.Turn, false) ? "You have no allies." : "Not a friendly hex.";
        }
        if (this.game.EditObj.LayerSupplyOn)
          this.SupplyLayerButtonId = 1;
        if (this.game.Data.Round > 0)
        {
          if (this.game.SelectX > -1 & this.game.SelectY > -1)
          {
            if (this.game.EditObj.UnitSelected > -1)
            {
              if (this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime == this.game.Data.Turn)
                this.HqUnitButtonId = 1;
              if (this.game.Data.Turn > -1)
              {
                if ( this.game.Data.RuleVar[528] == 1.0 && this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime == this.game.Data.Turn)
                  this.GiveUnitId = 1;
                if (this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime == this.game.Data.Turn)
                {
                  if (!this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected) & !this.game.HandyFunctionsObj.HasUnitNavySF(this.game.EditObj.UnitSelected))
                  {
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y].get_APPenalty(this.game.Data.Turn) > 0)
                    {
                      this.strategicbuttontext = "You cant strategically transfer a unit from a hex you conquered in this turn.";
                      this.groupstrategictext = "You cant strategically transfer a unit from a hex you conquered in this turn.";
                    }
                    else if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].DidAttack)
                    {
                      this.StrategicButtonId = 1;
                      if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].StartSize > -1)
                      {
                        let mut num: i32 =  1;
                        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                        for (let mut index: i32 =  0; index <= unitCounter; index += 1)
                        {
                          if (this.game.Data.UnitObj[index].Historical == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical & this.game.Data.UnitObj[index].Regime == this.game.Data.Turn && this.game.Data.UnitObj[index].DidAttack)
                            num = 0;
                        }
                        if (num == 1)
                          this.GroupStrategicButtonId = 1;
                      }
                    }
                    else
                    {
                      this.strategicbuttontext = "You cannot strategic transfer a unit that participated in an attack.";
                      this.groupstrategictext = "You cannot strategic transfer a unit that participated in an attack.";
                    }
                  }
                  else
                  {
                    this.groupstrategictext = "You cannot strategic transfer units with air or navy subformations.";
                    this.strategicbuttontext = "You cannot strategic transfer units with air or navy subformations.";
                  }
                }
              }
            }
            if (this.game.HandyFunctionsObj.VisibleEnemyUnitsInHex(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.Data.Turn, testingforattack: true) & this.game.Data.Round > 0)
            {
              this.AttackButtonId = 1;
              this.ArtAttackButtonId = 1;
              this.seaAttackButtonId = 1;
              this.SeaArtAttackButtonId = 1;
              this.AirAttackButtonId = 1;
              if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea)
              {
                this.SeaArtAttackButtonId = 0;
                this.SeaArtAttackButtonText = "You can only shore bombardment land hexes.";
              }
            }
            else
            {
              str: String = "No enemy unit in hex.";
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].RegimeRel[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime] >= 1)
                str = "Cannot attack. You need to declare war before you can attack";
              this.AttackButtonText = str;
              this.ArtAttackButtonText = str;
              this.SeaAttackButtonText = str;
              this.SeaArtAttackButtonText = str;
              this.AirAttackButtonText = str;
              if ( this.game.Data.RuleVar[318] > 0.0)
                this.AirAttackButtonId = 1;
            }
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].RegimeRel[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime] == 0)
            {
              this.ArtAttackButtonId = 1;
              this.SeaArtAttackButtonId = 1;
            }
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_SeeNow(this.game.Data.Turn) > 0)
            {
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime != this.game.Data.Turn)
              {
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 | this.game.HandyFunctionsObj.HasHexBridge(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected))
                {
                  if (this.game.HandyFunctionsObj.IsHostileNotSelf(this.game.Data.Turn, this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime))
                    this.InterdictButtonId = 1;
                  else
                    this.interdictbuttontext = "You are not at war with this regime. So you cannot bomb.";
                }
                else
                  this.interdictbuttontext = "No location or bridge in this hex to bomb.";
              }
              else
                this.interdictbuttontext = "You cannot bomb your own territory.";
            }
            else
              this.InterdictButtonId = 1;
          }
          else
          {
            this.AttackButtonText = "No hex selected.";
            this.ArtAttackButtonText = "No hex selected.";
            this.SeaAttackButtonText = "No hex selected.";
            this.SeaArtAttackButtonText = "No hex selected.";
            this.AirAttackButtonText = "No hex selected.";
            this.interdictbuttontext = "No hex selected.";
          }
        }
        if (this.game.SelectX > -1 & this.game.SelectY > -1)
        {
          if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime, this.game.Data.Turn) & this.game.Data.Round != 0)
          {
            this.NewUnitButtonId = 1;
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime != this.game.Data.Turn & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_ReconPts(this.game.Data.Turn) < 1)
            {
              this.NewUnitButtonId = 0;
              this.NewUnitButton2Id = 0;
              this.NewUnitButtonText = "You must have recon on the hex you want to create a unit.";
              this.newunitbutton2text = "You must have recon on the hex you want to create a unit.";
            }
          }
          else
          {
            this.NewUnitButtonText = "You can only make a new unit on a friendly hex.";
            this.newunitbutton2text = "You can only make a new unit on a friendly hex.";
          }
        }
        else
        {
          this.NewUnitButtonText = "No hex selected";
          this.newunitbutton2text = "No hex selected";
        }
        if ( this.game.Data.RuleVar[527] == 0.0)
        {
          this.newunitbutton2text = "";
          this.NewUnitButton2Id = 0;
        }
        if (this.game.Data.Round > 0)
          this.QuitID = 1;
        this.SaveId = 1;
        if (this.game.SelectX > -1 & this.game.SelectY > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 && this.game.SelectX > -1 & this.game.SelectY > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn | this.game.Data.Round == 0 && this.game.HandyFunctionsObj.CanLocProduce(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location, this.game.Data.Turn))
        {
          this.HqProdButtonId = 1;
          this.ProdButtonId = 1;
        }
        if (this.HqProdButtonId == 0)
          this.hqprodbuttontext = "You have to select a friendly location that can produce stuff.";
        if (this.ProdButtonId == 0)
          this.prodbuttontext = "You have to select a friendly location that can produce stuff.";
        if (this.game.SelectX > -1 & this.game.SelectY > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 && this.game.SelectX > -1 & this.game.SelectY > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn | this.game.Data.Round == 0 && this.game.HandyFunctionsObj.CanLocProduce(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location, this.game.Data.Turn) && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].NoHQ)
        {
          this.HqProdButtonId = 0;
          this.ProdButtonId = 0;
          this.hqprodbuttontext = "This location does not require a HQ";
          this.prodbuttontext = "Production for this location is automaticly set.";
        }
        num1: i32;
        if (this.game.Data.Round > 0 && this.game.SelectX > -1 & this.game.SelectY > -1)
        {
          if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter == -1)
          {
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime != this.game.Data.Turn)
              num1 = 1;
          }
          else
            num1 = !(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > -1 & this.game.EditObj.UnitSelected > -1) ? 1 : (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime != this.game.Data.Turn ? 1 : 0);
        }
        num2: i32;
        num3: i32;
        if (num1 == 0)
        {
          if (this.MoveButtonId >= 0)
            num3 = num2 + 1;
          if ( this.game.Data.RuleVar[344] > 0.0 &  this.game.Data.RuleVar[533] == 0.0 & this.GroupMoveButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[521] == 0.0 & this.HqUnitButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[519] == 0.0 & this.TransferButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[520] == 0.0 & this.StrategicButtonId >= 0)
            num3 += 1;
          if (this.GroupStrategicButtonId >= 0 &  this.game.Data.RuleVar[344] > 0.0 &  this.game.Data.RuleVar[533] == 0.0)
            num3 += 1;
          if ( this.game.Data.RuleVar[515] == 0.0 & this.ParadropButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[507] == 0.0 & this.LoadButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[507] == 0.0 & this.UnLoadButtonID >= 0)
            num3 += 1;
          if (this.NewUnitButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[518] == 0.0 & this.disbandid >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[522] == 0.0 & this.AirReconButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[516] == 0.0 & this.AirSupplyButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[505] == 0.0 & this.BlowBridgeButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[506] == 0.0 & this.BlowLocationButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[503] == 0.0 & this.InfraButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[504] == 0.0 & this.BuildButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[523] == 0.0 & this.SupplyLayerButtonId >= 0)
            num3 += 1;
          if (this.HexUnitButtonId >= 0)
            num3 += 1;
          if (this.ButtonZoomInId >= 0)
            num3 += 1;
          if (this.ButtonZoomOutId >= 0)
            num3 += 1;
          if (this.ButtonStackedUnitId >= 0)
            num3 += 1;
          if (this.NewUnitButton2Id >= 0 &  this.game.Data.RuleVar[527] > 0.0)
            num3 += 1;
          if ( this.game.Data.RuleVar[512] == 0.0 & this.NewUnitButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[529] == 1.0 & this.GiveHexId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[528] == 1.0 & this.GiveUnitId >= 0)
            num3 += 1;
          if (this.OfficerId > 0)
            num3 += 1;
          if (this.ChangeModelId > 0 &  this.game.Data.RuleVar[531] == 1.0)
            num3 += 1;
        }
        else
        {
          if (this.AttackButtonId >= 0)
            num3 = num2 + 1;
          if (this.ArtAttackButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[511] == 0.0 & this.seaAttackButtonId >= 0)
            num3 += 1;
          if ( this.game.Data.RuleVar[511] == 0.0 & this.SeaArtAttackButtonId >= 0)
            num3 += 1;
          if (this.AirAttackButtonId >= 0)
            num3 += 1;
          if (this.InterdictButtonId >= 0)
            num3 += 1;
          if (this.SupplyLayerButtonId >= 0)
            num3 += 1;
          if (this.HexUnitButtonId >= 0)
            num3 += 1;
          if (this.HqUnitButtonId >= 0)
            num3 += 1;
          if (this.NewUnitButton2Id >= 0 &  this.game.Data.RuleVar[527] > 0.0)
            num3 += 1;
          if (this.NewUnitButtonId >= 0)
            num3 += 1;
          if (this.OfficerId > 0)
            num3 += 1;
          if (this.ChangeModelId > 0)
            num3 += 1;
          if (this.StrategicButtonId > 0)
            num3 += 1;
          if (this.ButtonZoomInId >= 0)
            num3 += 1;
          if (this.ButtonZoomOutId >= 0)
            num3 += 1;
          if (this.ButtonStackedUnitId >= 0)
            num3 += 1;
          if (this.GiveUnitId > 0)
            num3 += 1;
        }
        let mut num4: i32 =   Math.Round( this.game.ScreenWidth / 2.0 -  (num3 * 18) + 30.0);
        num5: i32;
        num6: i32;
        if (this.NextButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 50))
        {
          num6 = num5 + 1;
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONNEXT, tDescript: "End your Turn", tBackbitmap: ( this.OwnBitmap), bbx: (this.game.ScreenWidth - 2 - 32 * num6), bby: 4);
          this.NextButtonId = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num6, 4, 30, 30, 1);
        }
        else
        {
          num6 = num5 + 1;
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONNEXT, 1, tBackbitmap: ( this.OwnBitmap), bbx: (this.game.ScreenWidth - 2 - 32 * num6), bby: 4);
          this.NextButtonId = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num6, 4, 30, 30, 0);
        }
        num7: i32;
        if (this.ResearchId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 23))
        {
          num7 = num6 + 1;
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONRESEARCH, tDescript: "Decision Room [F2]", tBackbitmap: ( this.OwnBitmap), bbx: (this.game.ScreenWidth - 2 - 32 * num7), bby: 4);
          this.ResearchId = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num7, 4, 30, 30, 1);
        }
        else
        {
          num7 = num6 + 1;
          if (this.game.EditObj.TutOrder > -1)
            this.researchtext = "";
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONRESEARCH, 1, this.researchtext,  this.OwnBitmap, this.game.ScreenWidth - 2 - 32 * num7, 4);
          this.ResearchId2 = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num7, 4, 30, 30, 0);
        }
        if ( this.game.Data.RuleVar[534] == 1.0 && this.SFDesignButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 52))
        {
          num7 += 1;
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONDESIGNSF, tDescript: "Subformationtype Model Design", tBackbitmap: ( this.OwnBitmap), bbx: (this.game.ScreenWidth - 2 - 32 * num7), bby: 4);
          this.SFDesignButtonId = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num7, 4, 30, 30, 1);
        }
        num8: i32;
        if (this.DipId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 24))
        {
          num8 = num7 + 1;
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONDIP, tDescript: "Strategic Information Map [F1]", tBackbitmap: ( this.OwnBitmap), bbx: (this.game.ScreenWidth - 2 - 32 * num8), bby: 4);
          this.DipId = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num8, 4, 30, 30, 1);
        }
        else
        {
          num8 = num7 + 1;
          if (this.game.EditObj.TutOrder > -1)
            this.diptext = "";
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONDIP, 1, this.diptext,  this.OwnBitmap, this.game.ScreenWidth - 2 - 32 * num8, 4);
          this.DipId2 = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num8, 4, 30, 30, 0);
        }
        if ( this.game.Data.RuleVar[531] > 0.0 &  this.game.Data.RuleVar[532] > 0.0 && this.ModelDesignerId > 0)
        {
          num8 += 1;
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONMODELDESIGNER, tDescript: "Model Designer", tBackbitmap: ( this.OwnBitmap), bbx: (this.game.ScreenWidth - 2 - 32 * num8), bby: 4);
          this.ModelDesignerId = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num8, 4, 30, 30, 1);
        }
        num9: i32;
        if (this.HistoryId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 26))
        {
          num9 = num8 + 1;
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONHISTORY, tDescript: "History [F4]", tBackbitmap: ( this.OwnBitmap), bbx: (this.game.ScreenWidth - 2 - 32 * num9), bby: 4);
          this.HistoryId = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num9, 4, 30, 30, 1);
        }
        else
        {
          num9 = num8 + 1;
          if (this.game.EditObj.TutOrder > -1)
            this.historytext = "";
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONHISTORY, 1, this.historytext,  this.OwnBitmap, this.game.ScreenWidth - 2 - 32 * num9, 4);
          this.HistoryId2 = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num9, 4, 30, 30, 0);
        }
        if (this.OrderSurrenderButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 41))
        {
          let mut num10: i32 =  num9 + 1;
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONSURRENDER, tDescript: "Surrender", tBackbitmap: ( this.OwnBitmap), bbx: (this.game.ScreenWidth - 2 - 32 * num10), bby: 4);
          this.OrderSurrenderButtonId = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num10, 4, 30, 30, 1);
        }
        else
        {
          let mut num11: i32 =  num9 + 1;
          if (this.game.EditObj.TutOrder > -1)
            this.ordersurrendertext = "";
          let mut tsubpart: SubPartClass =  new SteveButtonPartClass30b(this.game.BUTTONSURRENDER, 1, this.ordersurrendertext,  this.OwnBitmap, this.game.ScreenWidth - 2 - 32 * num11, 4);
          this.OrderSurrenderButtonId = this.AddSubPart( tsubpart, this.game.ScreenWidth - 2 - 32 * num11, 4, 30, 30, 0);
        }
        num12: i32;
        SubPartClass tsubpart1;
        if (num1 == 0)
        {
          num13: i32;
          if (this.MoveButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 1))
          {
            num13 = num12 + 1;
            let mut tsubpart2: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONMOVE, tDescript: "Move Unit [M]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
            this.MoveButtonId = this.AddSubPart( tsubpart2, num4 + num13 * 32, 4, 30, 30, 1);
          }
          else
          {
            num13 = num12 + 1;
            if (this.game.EditObj.TutOrder > -1)
              this.MoveButtonText = "";
            let mut tsubpart3: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONMOVE, 1, this.MoveButtonText,  this.OwnBitmap, num4 + num13 * 32, 4);
            this.MoveButtonId2 = this.AddSubPart( tsubpart3, num4 + num13 * 32, 4, 30, 30, 0);
          }
          if ( this.game.Data.RuleVar[533] == 0.0 &  this.game.Data.RuleVar[344] > 0.0)
          {
            if ( this.game.Data.RuleVar[344] > 0.0 & this.GroupMoveButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 48))
            {
              num13 += 1;
              let mut tsubpart4: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONMOVE2, tDescript: "Group Move Unit [G]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.GroupMoveButtonId = this.AddSubPart( tsubpart4, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.GroupMoveButtonText = "";
              let mut tsubpart5: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONMOVE2, 1, this.GroupMoveButtonText,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.GroupMoveButtonId2 = this.AddSubPart( tsubpart5, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[519] == 0.0)
          {
            if (this.TransferButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 9))
            {
              num13 += 1;
              let mut tsubpart6: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONTRANSFER, tDescript: "Transfer [T]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.TransferButtonId = this.AddSubPart( tsubpart6, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.TransferButtonText = "";
              let mut tsubpart7: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONTRANSFER, 1, this.TransferButtonText,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.TransferButtonId2 = this.AddSubPart( tsubpart7, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[520] == 0.0)
          {
            if (this.StrategicButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 18))
            {
              num13 += 1;
              let mut tsubpart8: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONSTRATEGIC, tDescript: "Strategic Transfer [S]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.StrategicButtonId = this.AddSubPart( tsubpart8, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.strategicbuttontext = "";
              let mut tsubpart9: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONSTRATEGIC, 1, this.strategicbuttontext,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.StrategicButtonId2 = this.AddSubPart( tsubpart9, num4 + num13 * 32, 4, 30, 30, 0);
            }
            if ( this.game.Data.RuleVar[533] == 0.0 &  this.game.Data.RuleVar[344] > 0.0)
            {
              if (this.GroupStrategicButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 49))
              {
                num13 += 1;
                let mut tsubpart10: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONSTRATEGIC2, tDescript: "Group Strategic Transfer", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
                this.GroupStrategicButtonId = this.AddSubPart( tsubpart10, num4 + num13 * 32, 4, 30, 30, 1);
              }
              else
              {
                num13 += 1;
                if (this.game.EditObj.TutOrder > -1)
                  this.groupstrategictext = "";
                let mut tsubpart11: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONSTRATEGIC2, 1, this.groupstrategictext,  this.OwnBitmap, num4 + num13 * 32, 4);
                this.GroupStrategicButtonId2 = this.AddSubPart( tsubpart11, num4 + num13 * 32, 4, 30, 30, 0);
              }
            }
          }
          if ( this.game.Data.RuleVar[521] == 0.0 | this.game.Data.Round == 0)
          {
            if (this.HqUnitButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 3))
            {
              num13 += 1;
              let mut tsubpart12: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONHQUNIT, tDescript: "Set Units HQ [H]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.HqUnitButtonId = this.AddSubPart( tsubpart12, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.HqUnitButtonText = "";
              let mut tsubpart13: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONHQUNIT, 1, this.HqUnitButtonText,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.HqUnitButtonId2 = this.AddSubPart( tsubpart13, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[515] == 0.0)
          {
            if (this.ParadropButtonId > 0)
            {
              num13 += 1;
              let mut tsubpart14: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONPARADROP, tDescript: "Use this Air Unit to do a paradrop", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.ParadropButtonId = this.AddSubPart( tsubpart14, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              let mut tsubpart15: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONPARADROP, 1, this.paradropbuttontext,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.ParadropButtonId2 = this.AddSubPart( tsubpart15, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[516] == 0.0)
          {
            if (this.AirSupplyButtonId > 0)
            {
              num13 += 1;
              let mut tsubpart16: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONAIRSUPPLY, tDescript: "Use this Air Unit to do an airsupply", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.AirSupplyButtonId = this.AddSubPart( tsubpart16, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              let mut tsubpart17: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONAIRSUPPLY, 1, this.airsupplybuttontext,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.AirSupplyButtonId2 = this.AddSubPart( tsubpart17, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[507] == 0.0)
          {
            if (this.LoadButtonId > 0)
            {
              num13 += 1;
              let mut tsubpart18: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONLOAD, tDescript: "Load Unit aboard this Naval Unit [L]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.LoadButtonId = this.AddSubPart( tsubpart18, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              let mut tsubpart19: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONLOAD, 1, this.loadbuttontext,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.LoadButtonId2 = this.AddSubPart( tsubpart19, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[507] == 0.0)
          {
            if (this.UnLoadButtonID > 0)
            {
              num13 += 1;
              let mut tsubpart20: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONUNLOAD, tDescript: "Unload this Unit [U]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.UnLoadButtonID = this.AddSubPart( tsubpart20, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              let mut tsubpart21: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONUNLOAD, 1, this.unloadbuttontext,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.UnLoadButtonID2 = this.AddSubPart( tsubpart21, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[512] == 0.0)
          {
            if (this.NewUnitButtonId > 0)
            {
              num13 += 1;
              let mut tsubpart22: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONNEWUNIT, tDescript: "Make New Unit [N]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.NewUnitButtonId = this.AddSubPart( tsubpart22, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              let mut tsubpart23: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONNEWUNIT, 1, this.NewUnitButtonText,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.NewUnitButtonId2 = this.AddSubPart( tsubpart23, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[527] > 0.0)
          {
            if (this.NewUnitButton2Id > 0)
            {
              num13 += 1;
              let mut tsubpart24: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONNEWUNIT2, tDescript: "Sub Unit Options (add, remove, change)", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.NewUnitButton2Id = this.AddSubPart( tsubpart24, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              let mut tsubpart25: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONNEWUNIT2, 1, this.NewUnitButtonText,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.NewUnitButton2Id2 = this.AddSubPart( tsubpart25, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[518] == 0.0)
          {
            if (this.disbandid > 0)
            {
              num13 += 1;
              let mut tsubpart26: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONDISBAND, tDescript: "Click to Disband Unit", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.disbandid = this.AddSubPart( tsubpart26, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              let mut tsubpart27: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONDISBAND, 1, this.disbandtext,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.disbandid2 = this.AddSubPart( tsubpart27, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[522] == 0.0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 33))
          {
            if (this.AirReconButtonId > 0)
            {
              num13 += 1;
              let mut tsubpart28: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONAIRRECON, tDescript: "Click to do an Air Recon Mission", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.AirReconButtonId = this.AddSubPart( tsubpart28, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.AirReconButtonText = "";
              let mut tsubpart29: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONAIRRECON, 1, this.AirReconButtonText,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.AirReconButtonId2 = this.AddSubPart( tsubpart29, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[505] == 0.0)
          {
            if (this.BlowBridgeButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 35))
            {
              num13 += 1;
              let mut tsubpart30: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONBLOWBRIDGE, tDescript: "Click to blow a bridge", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.BlowBridgeButtonId = this.AddSubPart( tsubpart30, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.blowbridgebuttontext = "";
              let mut tsubpart31: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONBLOWBRIDGE, 1, this.blowbridgebuttontext,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.BlowBridgeButtonId2 = this.AddSubPart( tsubpart31, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[506] == 0.0)
          {
            if (this.BlowLocationButtonId > 0)
            {
              num13 += 1;
              let mut tsubpart32: SubPartClass =  new SteveButtonPartClass30(this.game.BLOWLOCATIONBUTTON, tDescript: ("Click to blow the location. Will do between 0 and " + Conversion.Str( this.game.HandyFunctionsObj.GetBlowBridgePts(this.game.EditObj.UnitSelected)) + " points of structural damage."), tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.BlowLocationButtonId = this.AddSubPart( tsubpart32, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              let mut tsubpart33: SubPartClass =  new SteveButtonPartClass30(this.game.BLOWLOCATIONBUTTON, 1, this.blowlocationtext,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.BlowLocationButtonId2 = this.AddSubPart( tsubpart33, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[503] == 0.0)
          {
            if (this.InfraButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 36))
            {
              num13 += 1;
              let mut tsubpart34: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONBUILDROAD, tDescript: "Click to build road/bridge [R]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.InfraButtonId = this.AddSubPart( tsubpart34, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              if (this.game.EditObj.TutMode)
                this.infrabuttontext = "";
              let mut tsubpart35: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONBUILDROAD, 1, this.infrabuttontext,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.InfraButtonId2 = this.AddSubPart( tsubpart35, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[504] == 0.0)
          {
            if (this.BuildButtonId > 0)
            {
              num13 += 1;
              let mut tsubpart36: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONBUILDLOCATION, tDescript: "Click to build/repair a location", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.BuildButtonId = this.AddSubPart( tsubpart36, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              let mut tsubpart37: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONBUILDLOCATION, 1, this.buildbuttontext,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.BuildButtonId2 = this.AddSubPart( tsubpart37, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[528] == 1.0)
          {
            if (this.GiveUnitId > 0)
            {
              num13 += 1;
              let mut tsubpart38: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONGIVEUNIT, tDescript: "Click to give a unit to an ally (or whole HQ group)", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.GiveUnitId = this.AddSubPart( tsubpart38, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              if (this.game.EditObj.TutMode)
                this.GiveUnitText = "";
              let mut tsubpart39: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONGIVEUNIT, 1, this.GiveUnitText,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.GiveUnitId2 = this.AddSubPart( tsubpart39, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[529] == 1.0)
          {
            if (this.GiveHexId > 0)
            {
              num13 += 1;
              let mut tsubpart40: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONGIVEHEX, tDescript: "Click to give this hex or a group of hexes with this hex as center.", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
              this.GiveHexId = this.AddSubPart( tsubpart40, num4 + num13 * 32, 4, 30, 30, 1);
            }
            else
            {
              num13 += 1;
              let mut tsubpart41: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONGIVEHEX, 1, this.GiveHexText,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.GiveHexId2 = this.AddSubPart( tsubpart41, num4 + num13 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[523] == 0.0)
          {
            if (this.SupplyLayerButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 51))
            {
              num13 += 1;
              if (this.game.EditObj.LayerSupplyOn)
              {
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSUPPLYON, tDescript: "Click to turn off Supply layer [F5]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
                this.SupplyLayerButtonId = this.AddSubPart( tsubpart1, num4 + 32 * num13, 4, 30, 30, 1);
              }
              else
              {
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSUPPLYOFF, tDescript: "Click to turn on supply layer [F5]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num13 * 32), bby: 4);
                this.SupplyLayerButtonId = this.AddSubPart( tsubpart1, num4 + 32 * num13, 4, 30, 30, 1);
              }
            }
            else
            {
              num13 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.supplylayerbuttontext = "";
              let mut tsubpart42: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONSUPPLYON, 1, this.supplylayerbuttontext,  this.OwnBitmap, num4 + num13 * 32, 4);
              this.SupplyLayerButtonId2 = this.AddSubPart( tsubpart42, num4 + 32 * num13, 4, 30, 30, 0);
            }
          }
          num14: i32;
          if (this.HexUnitButtonId > 0 & this.game.EditObj.TutOrder == -1)
          {
            num14 = num13 + 1;
            if (this.game.EditObj.HideUnit == 0)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONHEX, tDescript: "Click to show units [F6]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num14 * 32), bby: 4);
              this.HexUnitButtonId = this.AddSubPart( tsubpart1, num4 + 32 * num14, 4, 30, 30, 1);
            }
            else if (this.game.EditObj.HideUnit == 1)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONHEXUNIT, tDescript: "Click to show different markers [F6]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num14 * 32), bby: 4);
              this.HexUnitButtonId = this.AddSubPart( tsubpart1, num4 + 32 * num14, 4, 30, 30, 1);
            }
            else
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONHEXUNIT2, tDescript: "Click to hide units [F6]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num14 * 32), bby: 4);
              this.HexUnitButtonId = this.AddSubPart( tsubpart1, num4 + 32 * num14, 4, 30, 30, 1);
            }
          }
          else
          {
            num14 = num13 + 1;
            tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONHEX, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num14 * 32), bby: 4);
            this.HexUnitButtonId2 = this.AddSubPart( tsubpart1, num4 + 32 * num14, 4, 30, 30, 0);
          }
          num15: i32;
          if (this.ButtonZoomInId > 0 & this.game.EditObj.TutOrder == -1)
          {
            num15 = num14 + 1;
            if (this.game.EditObj.Zoom < 1)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMIN, tDescript: "Click to zoom in", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num15 * 32), bby: 4);
              this.ButtonZoomInId = this.AddSubPart( tsubpart1, num4 + 32 * num15, 4, 30, 30, 1);
            }
            else if (this.game.EditObj.Zoom == 1)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMIN, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num15 * 32), bby: 4);
              this.ButtonZoomInId2 = this.AddSubPart( tsubpart1, num4 + 32 * num15, 4, 30, 30, 0);
            }
          }
          else
          {
            num15 = num14 + 1;
            tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMIN, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num15 * 32), bby: 4);
            this.ButtonZoomInId2 = this.AddSubPart( tsubpart1, num4 + 32 * num15, 4, 30, 30, 0);
          }
          num16: i32;
          if (this.ButtonZoomOutId > 0 & this.game.EditObj.TutOrder == -1)
          {
            num16 = num15 + 1;
            if (this.game.EditObj.Zoom > -1)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMOUT, tDescript: "Click to zoom out", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num16 * 32), bby: 4);
              this.ButtonZoomOutId = this.AddSubPart( tsubpart1, num4 + 32 * num16, 4, 30, 30, 1);
            }
            else if (this.game.EditObj.Zoom == -1)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMOUT, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num16 * 32), bby: 4);
              this.ButtonZoomOutId2 = this.AddSubPart( tsubpart1, num4 + 32 * num16, 4, 30, 30, 0);
            }
          }
          else
          {
            num16 = num15 + 1;
            tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMOUT, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num16 * 32), bby: 4);
            this.ButtonZoomOutId2 = this.AddSubPart( tsubpart1, num4 + 32 * num16, 4, 30, 30, 0);
          }
          num17: i32;
          if (this.ButtonStackedUnitId > 0 & this.game.EditObj.TutOrder == -1 & this.game.EditObj.Zoom == 1)
          {
            num17 = num16 + 1;
            if (!this.game.EditObj.SpreadUnit)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSPREADUNIT, tDescript: "Click to spread out units in zoomed in mode [F7]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num17 * 32), bby: 4);
              this.ButtonStackedUnitId = this.AddSubPart( tsubpart1, num4 + 32 * num17, 4, 30, 30, 1);
            }
            else
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSTACKEDUNIT, tDescript: "Click to stack units in zoomed in mode [F7]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num17 * 32), bby: 4);
              this.ButtonStackedUnitId = this.AddSubPart( tsubpart1, num4 + 32 * num17, 4, 30, 30, 1);
            }
          }
          else
          {
            num17 = num16 + 1;
            tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSPREADUNIT, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num17 * 32), bby: 4);
            this.ButtonStackedUnitId2 = this.AddSubPart( tsubpart1, num4 + 32 * num17, 4, 30, 30, 0);
          }
          if ( this.game.Data.RuleVar[343] == 1.0)
          {
            if (this.OfficerId > 0)
            {
              num17 += 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONOFFICER, tDescript: "Click to go to officerpool", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num17 * 32), bby: 4);
              this.OfficerId = this.AddSubPart( tsubpart1, num4 + 32 * num17, 4, 30, 30, 1);
            }
            else
            {
              num17 += 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONOFFICER, 1, this.OfficerText,  this.OwnBitmap, num4 + num17 * 32, 4);
              this.OfficerId2 = this.AddSubPart( tsubpart1, num4 + 32 * num17, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[531] == 1.0)
          {
            if (this.ChangeModelId > 0)
            {
              let mut num18: i32 =  num17 + 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONCHANGEMODEL, tDescript: "Click to change model of this unit", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num18 * 32), bby: 4);
              this.ChangeModelId = this.AddSubPart( tsubpart1, num4 + 32 * num18, 4, 30, 30, 1);
            }
            else
            {
              let mut num19: i32 =  num17 + 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONCHANGEMODEL, 1, this.ChangeModelText,  this.OwnBitmap, num4 + num19 * 32, 4);
              this.ChangeModelId2 = this.AddSubPart( tsubpart1, num4 + 32 * num19, 4, 30, 30, 0);
            }
          }
        }
        else
        {
          num20: i32;
          if (this.AttackButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 2))
          {
            num20 = num12 + 1;
            let mut tsubpart43: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONATTACK, tDescript: "Do a Land Attack on this Hex [A]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num20 * 32), bby: 4);
            this.AttackButtonId = this.AddSubPart( tsubpart43, num4 + num20 * 32, 4, 30, 30, 1);
          }
          else
          {
            num20 = num12 + 1;
            if (this.game.EditObj.TutOrder > -1)
              this.AttackButtonText = "";
            let mut tsubpart44: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONATTACK, 1, this.AttackButtonText,  this.OwnBitmap, num4 + num20 * 32, 4);
            this.AttackButtonId2 = this.AddSubPart( tsubpart44, num4 + num20 * 32, 4, 30, 30, 0);
          }
          num21: i32;
          if (this.ArtAttackButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 11))
          {
            num21 = num20 + 1;
            let mut tsubpart45: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONARTATTACK, tDescript: "Do an Artillery Barrage on this Hex [B]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num21 * 32), bby: 4);
            this.ArtAttackButtonId = this.AddSubPart( tsubpart45, num4 + num21 * 32, 4, 30, 30, 1);
          }
          else
          {
            num21 = num20 + 1;
            if (this.game.EditObj.TutOrder > -1)
              this.ArtAttackButtonText = "";
            let mut tsubpart46: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONARTATTACK, 1, this.ArtAttackButtonText,  this.OwnBitmap, num4 + num21 * 32, 4);
            this.ArtAttackButtonId2 = this.AddSubPart( tsubpart46, num4 + num21 * 32, 4, 30, 30, 0);
          }
          if ( this.game.Data.RuleVar[511] == 0.0)
          {
            num22: i32;
            if (this.seaAttackButtonId > 0)
            {
              num22 = num21 + 1;
              let mut tsubpart47: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONSEAATTACK, tDescript: "Do a Sea Attack on this Hex [A]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num22 * 32), bby: 4);
              this.seaAttackButtonId = this.AddSubPart( tsubpart47, num4 + num22 * 32, 4, 30, 30, 1);
            }
            else
            {
              num22 = num21 + 1;
              let mut tsubpart48: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONSEAATTACK, 1, this.SeaAttackButtonText,  this.OwnBitmap, num4 + num22 * 32, 4);
              this.seaAttackButtonId2 = this.AddSubPart( tsubpart48, num4 + num22 * 32, 4, 30, 30, 0);
            }
            if (this.SeaArtAttackButtonId > 0)
            {
              num21 = num22 + 1;
              let mut tsubpart49: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONSEAARTATTACK, tDescript: "Do Shorebombardment on this Hex", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num21 * 32), bby: 4);
              this.SeaArtAttackButtonId = this.AddSubPart( tsubpart49, num4 + num21 * 32, 4, 30, 30, 1);
            }
            else
            {
              num21 = num22 + 1;
              let mut tsubpart50: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONSEAARTATTACK, 1, this.SeaArtAttackButtonText,  this.OwnBitmap, num4 + num21 * 32, 4);
              this.SeaArtAttackButtonId2 = this.AddSubPart( tsubpart50, num4 + num21 * 32, 4, 30, 30, 0);
            }
          }
          num23: i32;
          if (this.AirAttackButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 14))
          {
            num23 = num21 + 1;
            let mut tsubpart51: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONAIRATTACK, tDescript: "Do an Airstrike on this Hex [Z]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num23 * 32), bby: 4);
            this.AirAttackButtonId = this.AddSubPart( tsubpart51, num4 + num23 * 32, 4, 30, 30, 1);
          }
          else
          {
            num23 = num21 + 1;
            if (this.game.EditObj.TutOrder > -1)
              this.AirAttackButtonText = "";
            let mut tsubpart52: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONAIRATTACK, 1, this.AirAttackButtonText,  this.OwnBitmap, num4 + num23 * 32, 4);
            this.AirAttackButtonId2 = this.AddSubPart( tsubpart52, num4 + num23 * 32, 4, 30, 30, 0);
          }
          if ( this.game.Data.RuleVar[517] == 0.0)
          {
            if (this.InterdictButtonId > 0)
            {
              num23 += 1;
              let mut tsubpart53: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONINTERDICT, tDescript: "Do a Bombing Raid on this Hex [X]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num23 * 32), bby: 4);
              this.InterdictButtonId = this.AddSubPart( tsubpart53, num4 + num23 * 32, 4, 30, 30, 1);
            }
            else
            {
              num23 += 1;
              let mut tsubpart54: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONINTERDICT, 1, this.interdictbuttontext,  this.OwnBitmap, num4 + num23 * 32, 4);
              this.InterdictButtonId2 = this.AddSubPart( tsubpart54, num4 + num23 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[523] == 0.0)
          {
            if (this.SupplyLayerButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 51))
            {
              num23 += 1;
              if (this.game.EditObj.LayerSupplyOn)
              {
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSUPPLYON, tDescript: "Click to turn off Supply layer [F5]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num23 * 32), bby: 4);
                this.SupplyLayerButtonId = this.AddSubPart( tsubpart1, num4 + 32 * num23, 4, 30, 30, 1);
              }
              else
              {
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSUPPLYOFF, tDescript: "Click to turn on supply layer [F5]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num23 * 32), bby: 4);
                this.SupplyLayerButtonId = this.AddSubPart( tsubpart1, num4 + 32 * num23, 4, 30, 30, 1);
              }
            }
            else
            {
              num23 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.supplylayerbuttontext = "";
              let mut tsubpart55: SubPartClass =  new SteveButtonPartClass30(this.game.BUTTONSUPPLYON, 1, this.supplylayerbuttontext,  this.OwnBitmap, num4 + num23 * 32, 4);
              this.SupplyLayerButtonId2 = this.AddSubPart( tsubpart55, num4 + 32 * num23, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[528] == 1.0)
          {
            if (this.GiveUnitId > 0)
            {
              num23 += 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONGIVEUNIT, tDescript: "Click to give a unit to an ally (or whole HQ group)", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num23 * 32), bby: 4);
              this.GiveUnitId = this.AddSubPart( tsubpart1, num4 + num23 * 32, 4, 30, 30, 1);
            }
            else
            {
              num23 += 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONGIVEUNIT, 1, this.GiveUnitText,  this.OwnBitmap, num4 + num23 * 32, 4);
              this.GiveUnitId2 = this.AddSubPart( tsubpart1, num4 + num23 * 32, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[521] == 0.0 | this.game.Data.Round == 0)
          {
            if (this.HqUnitButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 3))
            {
              num23 += 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONHQUNIT, tDescript: "Set Units HQ [H]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num23 * 32), bby: 4);
              this.HqUnitButtonId = this.AddSubPart( tsubpart1, num4 + num23 * 32, 4, 30, 30, 1);
            }
            else
            {
              num23 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.HqUnitButtonText = "";
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONHQUNIT, 1, this.HqUnitButtonText,  this.OwnBitmap, num4 + num23 * 32, 4);
              this.HqUnitButtonId2 = this.AddSubPart( tsubpart1, num4 + num23 * 32, 4, 30, 30, 0);
            }
          }
          num24: i32;
          if (this.HexUnitButtonId > 0)
          {
            num24 = num23 + 1;
            if (this.game.EditObj.HideUnit == 0)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONHEX, tDescript: "Click to show units [F6]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num24 * 32), bby: 4);
              this.HexUnitButtonId = this.AddSubPart( tsubpart1, num4 + 32 * num24, 4, 30, 30, 1);
            }
            else if (this.game.EditObj.HideUnit == 1)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONHEXUNIT, tDescript: "Click to show different markers [F6]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num24 * 32), bby: 4);
              this.HexUnitButtonId = this.AddSubPart( tsubpart1, num4 + 32 * num24, 4, 30, 30, 1);
            }
            else
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONHEXUNIT2, tDescript: "Click to hide units [F6]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num24 * 32), bby: 4);
              this.HexUnitButtonId = this.AddSubPart( tsubpart1, num4 + 32 * num24, 4, 30, 30, 1);
            }
          }
          else
          {
            num24 = num23 + 1;
            tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONHEX, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num24 * 32), bby: 4);
            this.HexUnitButtonId2 = this.AddSubPart( tsubpart1, num4 + 32 * num24, 4, 30, 30, 0);
          }
          num25: i32;
          if (this.ButtonZoomInId > 0 & this.game.EditObj.TutOrder == -1)
          {
            num25 = num24 + 1;
            if (this.game.EditObj.Zoom < 1)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMIN, tDescript: "Click to zoom in [+]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num25 * 32), bby: 4);
              this.ButtonZoomInId = this.AddSubPart( tsubpart1, num4 + 32 * num25, 4, 30, 30, 1);
            }
            else if (this.game.EditObj.Zoom == 1)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMIN, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num25 * 32), bby: 4);
              this.ButtonZoomInId2 = this.AddSubPart( tsubpart1, num4 + 32 * num25, 4, 30, 30, 0);
            }
          }
          else
          {
            num25 = num24 + 1;
            tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMIN, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num25 * 32), bby: 4);
            this.ButtonZoomInId2 = this.AddSubPart( tsubpart1, num4 + 32 * num25, 4, 30, 30, 0);
          }
          num26: i32;
          if (this.ButtonZoomOutId > 0 & this.game.EditObj.TutOrder == -1)
          {
            num26 = num25 + 1;
            if (this.game.EditObj.Zoom > -1)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMOUT, tDescript: "Click to zoom in [-]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num26 * 32), bby: 4);
              this.ButtonZoomOutId = this.AddSubPart( tsubpart1, num4 + 32 * num26, 4, 30, 30, 1);
            }
            else if (this.game.EditObj.Zoom == -1)
            {
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMOUT, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num26 * 32), bby: 4);
              this.ButtonZoomOutId2 = this.AddSubPart( tsubpart1, num4 + 32 * num26, 4, 30, 30, 0);
            }
          }
          else
          {
            num26 = num25 + 1;
            tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONZOOMOUT, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num26 * 32), bby: 4);
            this.ButtonZoomOutId2 = this.AddSubPart( tsubpart1, num4 + 32 * num26, 4, 30, 30, 0);
          }
          if (this.game.EditObj.Zoom == 1)
          {
            if (this.ButtonStackedUnitId > 0 & this.game.EditObj.TutOrder == -1)
            {
              num26 += 1;
              if (!this.game.EditObj.SpreadUnit)
              {
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSPREADUNIT, tDescript: "Click to spread out units in zoomed in mode [F7]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num26 * 32), bby: 4);
                this.ButtonStackedUnitId = this.AddSubPart( tsubpart1, num4 + 32 * num26, 4, 30, 30, 1);
              }
              else
              {
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSTACKEDUNIT, tDescript: "Click to stack units in zoomed in mode [F7]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num26 * 32), bby: 4);
                this.ButtonStackedUnitId = this.AddSubPart( tsubpart1, num4 + 32 * num26, 4, 30, 30, 1);
              }
            }
            else
            {
              num26 += 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSPREADUNIT, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num26 * 32), bby: 4);
              this.ButtonStackedUnitId2 = this.AddSubPart( tsubpart1, num4 + 32 * num26, 4, 30, 30, 0);
            }
          }
          if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime))
          {
            if ( this.game.Data.RuleVar[512] == 0.0)
            {
              if (this.NewUnitButtonId > 0)
              {
                num26 += 1;
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONNEWUNIT, tDescript: "Make New Unit [N]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num26 * 32), bby: 4);
                this.NewUnitButtonId = this.AddSubPart( tsubpart1, num4 + num26 * 32, 4, 30, 30, 1);
              }
              else
              {
                num26 += 1;
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONNEWUNIT, 1, this.NewUnitButtonText,  this.OwnBitmap, num4 + num26 * 32, 4);
                this.NewUnitButtonId2 = this.AddSubPart( tsubpart1, num4 + num26 * 32, 4, 30, 30, 0);
              }
            }
            if ( this.game.Data.RuleVar[527] > 0.0)
            {
              if (this.NewUnitButton2Id > 0)
              {
                num26 += 1;
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONNEWUNIT2, tDescript: "Make New Sub Unit", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num26 * 32), bby: 4);
                this.NewUnitButton2Id = this.AddSubPart( tsubpart1, num4 + num26 * 32, 4, 30, 30, 1);
              }
              else
              {
                num26 += 1;
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONNEWUNIT2, 1, this.NewUnitButtonText,  this.OwnBitmap, num4 + num26 * 32, 4);
                this.NewUnitButton2Id2 = this.AddSubPart( tsubpart1, num4 + num26 * 32, 4, 30, 30, 0);
              }
            }
          }
          if ( this.game.Data.RuleVar[343] == 1.0)
          {
            if (this.OfficerId > 0)
            {
              num26 += 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONOFFICER, tDescript: "Click to go to officerpool", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num26 * 32), bby: 4);
              this.OfficerId = this.AddSubPart( tsubpart1, num4 + 32 * num26, 4, 30, 30, 1);
            }
            else
            {
              num26 += 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONOFFICER, 1, this.OfficerText,  this.OwnBitmap, num4 + num26 * 32, 4);
              this.OfficerId2 = this.AddSubPart( tsubpart1, num4 + 32 * num26, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[531] == 1.0)
          {
            if (this.ChangeModelId > 0)
            {
              num26 += 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONCHANGEMODEL, tDescript: "Click to change model of this unit", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num26 * 32), bby: 4);
              this.ChangeModelId = this.AddSubPart( tsubpart1, num4 + 32 * num26, 4, 30, 30, 1);
            }
            else
            {
              num26 += 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONCHANGEMODEL, 1, this.ChangeModelText,  this.OwnBitmap, num4 + num26 * 32, 4);
              this.ChangeModelId2 = this.AddSubPart( tsubpart1, num4 + 32 * num26, 4, 30, 30, 0);
            }
          }
          if ( this.game.Data.RuleVar[520] == 0.0)
          {
            num27: i32;
            if (this.StrategicButtonId > 0)
            {
              num27 = num26 + 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSTRATEGIC, tDescript: "Strategic Transfer [S]", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num27 * 32), bby: 4);
              this.StrategicButtonId = this.AddSubPart( tsubpart1, num4 + num27 * 32, 4, 30, 30, 1);
            }
            else
            {
              num27 = num26 + 1;
              tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSTRATEGIC, 1, this.strategicbuttontext,  this.OwnBitmap, num4 + num27 * 32, 4);
              this.StrategicButtonId2 = this.AddSubPart( tsubpart1, num4 + num27 * 32, 4, 30, 30, 0);
            }
            if ( this.game.Data.RuleVar[533] == 0.0 &  this.game.Data.RuleVar[344] > 0.0)
            {
              if (this.GroupStrategicButtonId > 0)
              {
                let mut num28: i32 =  num27 + 1;
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSTRATEGIC2, tDescript: "Group Strategic Transfer", tBackbitmap: ( this.OwnBitmap), bbx: (num4 + num28 * 32), bby: 4);
                this.GroupStrategicButtonId = this.AddSubPart( tsubpart1, num4 + num28 * 32, 4, 30, 30, 1);
              }
              else
              {
                let mut num29: i32 =  num27 + 1;
                tsubpart1 =  new SteveButtonPartClass30(this.game.BUTTONSTRATEGIC2, 1, this.groupstrategictext,  this.OwnBitmap, num4 + num29 * 32, 4);
                this.GroupStrategicButtonId2 = this.AddSubPart( tsubpart1, num4 + num29 * 32, 4, 30, 30, 0);
              }
            }
          }
        }
        num30: i32;
        let mut num31: i32 =  num30 + 1;
        tsubpart1 =  new SteveButtonPartClass30(this.game.BACKBUTTON, 1, "Currently in main play window",  this.OwnBitmap, num31 * 32 - 30, 4);
        this.FakeBackButtonId = this.AddSubPart( tsubpart1, num31 * 32 - 32, 4, 30, 30, 0);
        if (this.StatisticsButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 32))
        {
          num31 += 1;
          tsubpart1 =  new SteveButtonPartClass30b(this.game.BUTTONSTATISTICS, tDescript: "Statistics [F3]", tBackbitmap: ( this.OwnBitmap), bbx: (num31 * 32 - 30), bby: 4);
          this.StatisticsButtonId = this.AddSubPart( tsubpart1, num31 * 32 - 32, 4, 30, 30, 1);
        }
        if (this.PrefsButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 30))
        {
          num31 += 1;
          tsubpart1 =  new SteveButtonPartClass30b(this.game.BUTTONPREFS, tDescript: "System Options (prefs,save,quit)", tBackbitmap: ( this.OwnBitmap), bbx: (num31 * 32 - 30), bby: 4);
          this.PrefsButtonId = this.AddSubPart( tsubpart1, num31 * 32 - 32, 4, 30, 30, 1);
        }
        if ( this.game.Data.RuleVar[513] == 0.0)
        {
          num32: i32;
          if (this.HqProdButtonId > 0)
          {
            num32 = num31 + 1;
            tsubpart1 =  new SteveButtonPartClass30b(this.game.BUTTONHQPROD, tDescript: "Set HQ of location [O]", tBackbitmap: ( this.OwnBitmap), bbx: (num32 * 32 - 32), bby: 4);
            this.HqProdButtonId = this.AddSubPart( tsubpart1, num32 * 32 - 32, 4, 30, 30, 1);
          }
          else
          {
            num32 = num31 + 1;
            tsubpart1 =  new SteveButtonPartClass30b(this.game.BUTTONHQPROD, 1, this.hqprodbuttontext,  this.OwnBitmap, num32 * 32 - 32, 4);
            this.HqProdButtonId2 = this.AddSubPart( tsubpart1, num32 * 32 - 32, 4, 30, 30, 0);
          }
          if (this.ProdButtonId > 0)
          {
            let mut num33: i32 =  num32 + 1;
            tsubpart1 =  new SteveButtonPartClass30b(this.game.BUTTONPROD, tDescript: "Set production of location [P]", tBackbitmap: ( this.OwnBitmap), bbx: (num33 * 32 - 32), bby: 4);
            this.ProdButtonId = this.AddSubPart( tsubpart1, num33 * 32 - 32, 4, 30, 30, 1);
          }
          else
          {
            let mut num34: i32 =  num32 + 1;
            tsubpart1 =  new SteveButtonPartClass30b(this.game.BUTTONPROD, 1, this.prodbuttontext,  this.OwnBitmap, num34 * 32 - 32, 4);
            this.ProdButtonId2 = this.AddSubPart( tsubpart1, num34 * 32 - 32, 4, 30, 30, 0);
          }
        }
      }
      else
      {
        str: String;
        if (this.game.EditObj.OrderType == 1)
          str = "Movement";
        if (this.game.EditObj.OrderType == 48)
          str = "Group Movement";
        if (this.game.EditObj.OrderType == 14)
          str = "Airstrike";
        if (this.game.EditObj.OrderType == 11)
          str = "Land Artillery Attack";
        if (this.game.EditObj.OrderType == 2)
          str = "Land Attack";
        if (this.game.EditObj.OrderType == 15)
          str = "Bombing Raid";
        if (this.game.EditObj.OrderType == 25)
          str = "Construction Menu";
        if (this.game.EditObj.OrderType == 24)
          str = "Strategic Information. You play " + this.game.Data.RegimeObj[this.game.Data.Turn].Name;
        if (this.game.EditObj.OrderType == 26)
          str = "History";
        if (this.game.EditObj.OrderType == 20)
          str = "Embark Unit";
        if (this.game.EditObj.OrderType == 8)
          str = "Create new Subformation/HQ";
        if (this.game.EditObj.OrderType == 7)
          str = "New Unit";
        if (this.game.EditObj.OrderType == 19)
          str = "Paradrop/Airferry";
        if (this.game.EditObj.OrderType == 42)
          str = "Airlift";
        if (this.game.EditObj.OrderType == 22)
          str = "Officerpool";
        if (this.game.EditObj.OrderType == 6)
          str = "Production";
        if (this.game.EditObj.OrderType == 4)
          str = "Locations HQ";
        if (this.game.EditObj.OrderType == 5)
          str = "Recruitment";
        if (this.game.EditObj.OrderType == 23)
          str = "Decision Room";
        if (this.game.EditObj.OrderType == 52)
          str = "Subformation Model Design";
        if (this.game.EditObj.OrderType == 13)
          str = "Shore Bombardment";
        if (this.game.EditObj.OrderType == 12)
          str = "Sea Attack";
        if (this.game.EditObj.OrderType == 18)
          str = "Strategic Transfer";
        if (this.game.EditObj.OrderType == 49)
          str = "Group Strategic Transfer";
        if (this.game.EditObj.OrderType == 9)
          str = "Transfer";
        if (this.game.EditObj.OrderType == 3)
          str = "Set Unit's HQ";
        if (this.game.EditObj.OrderType == 21)
          str = "Disembark Unit";
        if (this.game.EditObj.OrderType == 30)
          str = "System Options";
        if (this.game.EditObj.OrderType == 33)
          str = "Air Recon";
        if (this.game.EditObj.OrderType == 35)
          str = "Blow Bridge";
        if (this.game.EditObj.OrderType == 36)
          str = "Build Road/Bridge";
        if (this.game.EditObj.OrderType == 37)
          str = "Build/Repair Location";
        if (this.game.EditObj.OrderType == 39)
          str = "Select Anti-Supply Target";
        if (this.game.EditObj.OrderType == 40)
          str = "Air Supply";
        if (this.game.EditObj.OrderType == 43)
          str = "AI Testing, Pick Marker for unit";
        if (this.game.EditObj.OrderType == 44)
          str = "Make new subunit";
        if (this.game.EditObj.OrderType == 45)
          str = "Officer Pool";
        if (this.game.EditObj.OrderType == 46)
          str = "Change Model";
        if (this.game.EditObj.OrderType == 47)
          str = "Model Designer";
        bool flag1 = true;
        this.NotOkText = "Option not available";
        if (this.game.EditObj.OrderType == 6)
        {
          let mut Number: i32 =  this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.EditObj.OrderLoc].Type].MaxProd;
          if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.EditObj.OrderLoc].Type].StructuralPts > 0)
            Number =  Math.Round(Conversion.Int( Number * ( this.game.Data.LocObj[this.game.EditObj.OrderLoc].StructuralPts /  this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.EditObj.OrderLoc].Type].StructuralPts)));
          if (this.game.Data.Round <= 0)
            ;
          str = "Production for " + this.game.Data.LocObj[this.game.EditObj.OrderLoc].Name + " (" + Strings.Trim(Conversion.Str( Number)) + "points)";
          if (this.game.HandyFunctionsObj.GetProdTotalPercent(this.game.EditObj.OrderLoc) > 100)
            flag1 = false;
        }
        if (this.game.EditObj.OrderType == 9)
          str = this.game.EditObj.OrderTarget != -1 ? "Transfer subformations" : "Select target for transfers";
        bool flag2;
        if (this.game.EditObj.OrderType == 18 | this.game.EditObj.OrderType == 49)
        {
          if (this.game.EditObj.OrderTarget == -1)
          {
            this.PopupButtonId = 1;
            str = "Select HQ to provide Movement Capacity";
            if (this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ && this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
              flag2 = true;
          }
          else
            str = "Select destination Hex";
        }
        tDescript: String;
        if (this.game.EditObj.OrderType == 3)
        {
          str = "Select HQ for Unit";
          this.PopupButtonId = 1;
          if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical > -1)
          {
            if (this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.OrderUnit, true) > 0)
              str = str + ". Costs " + Conversion.Str( this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.OrderUnit, false)) + " PP - " + Conversion.Str( this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.OrderUnit, true)) + " PP to switch due to the negative PP ranking of its current commander.";
          }
          else if (this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.OrderUnit, false) > 0)
            str = str + ". Costs " + Conversion.Str( this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.OrderUnit, false)) + " PP due to the negative PP ranking of its current commander.";
          if (this.game.EditObj.UnitSelected > -1)
          {
            this.NotOkText = "Unit is not a HQ";
            if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
            {
              this.NotOkText = "You cannot select self as HQ";
              if (this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit)
              {
                this.NotOkText = "That is already the HQ";
                if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].HQ != this.game.EditObj.UnitSelected)
                {
                  this.NotOkText = "Unit is not a HQ";
                  if (this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit | this.game.Data.UnitObj[this.game.EditObj.OrderUnit].IsHQ)
                  {
                    this.NotOkText = "Unit cannot be set as HQ";
                    if (this.game.HandyFunctionsObj.CanUnitBecomeHQfor(this.game.EditObj.UnitSelected, this.game.EditObj.OrderUnit))
                    {
                      flag2 = true;
                      if ( this.game.Data.RuleVar[304] > 0.0)
                      {
                        let mut num: i32 =  0;
                        if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].IsHQ)
                          num = 1;
                        if ( (this.game.HandyFunctionsObj.HowmanyHQsAbove(this.game.EditObj.UnitSelected) + this.game.HandyFunctionsObj.HowmanyHQsBelow(this.game.EditObj.OrderUnit) + 1 + num) >  this.game.Data.RuleVar[304])
                        {
                          this.NotOkText = "Cannot select as HQ because it would exceed the maximum amount of " + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[304])) + " HQs in HQ chain. Current HQs above selected unit is " + Strings.Trim(Conversion.Str( this.game.HandyFunctionsObj.HowmanyHQsAbove(this.game.EditObj.UnitSelected)));
                          flag2 = false;
                        }
                      }
                      if (!( this.game.Data.RuleVar[344] == 1.0 &  this.game.Data.RuleVar[348] == 1.0) || this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical <= -1 || this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical <= -1 || this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical].Type < this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].Type)
                        ;
                    }
                  }
                }
              }
            }
          }
          this.KillId = 1;
          tDescript = "Set Unit to No HQ";
        }
        if (this.game.EditObj.OrderType == 35)
        {
          str = "Select Bridge Direction to blow";
          if (this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] == 0)
            flag2 = true;
        }
        if (this.game.EditObj.OrderType == 36)
        {
          str = "Select Direction to build road/bridge";
          if (this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] == 0)
            flag2 = true;
        }
        if (this.game.EditObj.OrderType == 19 | this.game.EditObj.OrderType == 42)
        {
          if (this.game.EditObj.OrderTarget == -1)
          {
            str = "Select Unit to paradrop or airlift";
            if (this.game.EditObj.UnitSelected > -1 & this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit)
            {
              this.NotOkText = "Unit is empty";
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount > -1)
              {
                this.NotOkText = "Unit is a HQ. Only non-HQs can be dropped.";
                if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
                {
                  this.NotOkText = "Unit must have at least 50ap";
                  if (this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) >= 50)
                  {
                    this.NotOkText = "Unit must be in same hex.";
                    if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X == this.game.SelectX & this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y == this.game.SelectY)
                    {
                      this.NotOkText = "Unit is not yours.";
                      if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
                      {
                        this.NotOkText = "Unit is to heavy to be carried.";
                        if (this.game.HandyFunctionsObj.GetCarryCapPts(this.game.EditObj.OrderUnit, 2) >= this.game.HandyFunctionsObj.GetUnitWeight(this.game.EditObj.UnitSelected))
                          flag2 = true;
                      }
                    }
                  }
                }
              }
            }
          }
          else if (this.game.EditObj.TargetX == -1)
          {
            str = this.game.EditObj.OrderType != 19 ? "Select airfield to airlift too" : "Select Hex to paradrop on";
            if (!(this.game.EditObj.OrderX == this.game.SelectX & this.game.EditObj.OrderY == this.game.SelectY & this.game.EditObj.OrderMap == this.game.EditObj.MapSelected))
            {
              this.NotOkText = "Hex cannot be reached.";
              if (this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] <= this.game.HandyFunctionsObj.GetLowestAirAp(this.game.EditObj.OrderUnit))
              {
                this.NotOkText = "Option not available..";
                if (this.game.HandyFunctionsObj.IsHostileNotSelf(this.game.Data.Turn, this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime))
                {
                  this.NotOkText = "Unit needs at least " + Conversion.Str( this.game.Data.RuleVar[307]) + " power points to capture enemy territory.";
                  if ( this.game.Data.RuleVar[307] <=  this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.game.EditObj.OrderTarget))
                  {
                    this.NotOkText = "Unit does not only have paratroopers.";
                    if (this.game.HandyFunctionsObj.HasUnitOnlyParaLandSF(this.game.EditObj.OrderTarget))
                      flag2 = true;
                  }
                }
                else
                {
                  this.NotOkText = "Not on sea";
                  if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea)
                  {
                    if (!this.game.HandyFunctionsObj.HasUnitOnlyParaLandSF(this.game.EditObj.OrderTarget))
                    {
                      this.NotOkText = "Must have location";
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1)
                      {
                        this.NotOkText = "Must have location with airfield";
                        if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].IsAirfield)
                          flag2 = true;
                      }
                    }
                    else
                      flag2 = true;
                  }
                }
              }
            }
          }
        }
        if (this.game.EditObj.OrderType == 33)
        {
          str = "Select target hex for recon mission";
          if (!(this.game.EditObj.OrderX == this.game.SelectX & this.game.EditObj.OrderY == this.game.SelectY & this.game.EditObj.MapSelected == this.game.EditObj.OrderMap) && this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] <= this.game.HandyFunctionsObj.GetLowestAirAp(this.game.EditObj.OrderUnit))
            flag2 = true;
        }
        if (this.game.EditObj.OrderType == 40)
        {
          str = "Select target hex for air supply";
          if (this.game.EditObj.TargetX > -1)
            str = "Air supply";
          if (!(this.game.EditObj.OrderX == this.game.SelectX & this.game.EditObj.OrderY == this.game.SelectY & this.game.EditObj.MapSelected == this.game.EditObj.OrderMap) && this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] <= this.game.HandyFunctionsObj.GetLowestAirAp(this.game.EditObj.OrderUnit) && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn && this.game.EditObj.TargetX == -1)
            flag2 = true;
        }
        bool flag3;
        if (this.game.EditObj.OrderType == 2)
        {
          str = "Select Land Attack participants. Attack Stack: " + Conversion.Str( (this.game.HandyFunctionsObj.CurrentAttackStack() + this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].get_BattleStack(this.game.Data.Turn))) + "/" + Conversion.Str( this.game.HandyFunctionsObj.maxAttackStack()) + ". ConcBonus: +" + Conversion.Str( Conversion.Int( (( this.game.HandyFunctionsObj.GetConcentricBonus2() - 1.0) * 100.0))) + "%";
          let mut divBonusForAttack: i32 =  this.game.HandyFunctionsObj.GetDivBonusForAttack(this.game.EditObj.TargetX, this.game.EditObj.TargetY, this.game.EditObj.TargetMap);
          if (divBonusForAttack > 0)
            str = str + ". DivBonus: +" + Conversion.Str( Conversion.Int(divBonusForAttack)) + "%";
          if (this.game.EditObj.UnitSelected > -1)
          {
            Coordinate target = Coordinate::new();
            target.x = this.game.EditObj.TargetX;
            target.y = this.game.EditObj.TargetY;
            target.map = this.game.EditObj.MapSelected;
            this.NotOkText = "Unit needs at least " + Conversion.Str( this.game.Data.RuleVar[307]) + " power points to join in attack.";
            if ( this.game.Data.RuleVar[307] <=  this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.game.EditObj.UnitSelected))
            {
              this.NotOkText = "Unit does not have enough AP to move into hex or one of its subformationtypes cannot move into this landscape type.";
              if (this.game.HandyFunctionsObj.CanDoLandAttack(this.game.EditObj.UnitSelected, target))
                flag2 = true;
            }
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 14)
        {
          this.PopupButtonId = 1;
          str = "Preparing Air Strike. Select participants. ";
          if ( this.game.Data.RuleVar[833] > 0.0)
            str = str + Conversion.Str( (this.game.HandyFunctionsObj.CurrentAttackStackAir() + this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].get_BattleStackAir(this.game.Data.Turn))) + "/" + Conversion.Str( this.game.Data.RuleVar[833]);
          if (this.game.EditObj.UnitSelected > -1)
          {
            if (this.game.HandyFunctionsObj.CanDoAirStrike(this.game.EditObj.UnitSelected, Coordinate::new()
            {
              x = this.game.EditObj.TargetX,
              y = this.game.EditObj.TargetY
            }))
              flag2 = true;
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 15)
        {
          this.PopupButtonId = 1;
          str = "Preparing Bombing Run. Select participants." + Conversion.Str( (this.game.HandyFunctionsObj.CurrentAttackStackAir() + this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].get_BattleStackAir(this.game.Data.Turn))) + "/" + Conversion.Str( this.game.Data.RuleVar[833]);
          if (this.game.EditObj.UnitSelected > -1)
          {
            if (this.game.HandyFunctionsObj.CanDoAirStrike(this.game.EditObj.UnitSelected, Coordinate::new()
            {
              x = this.game.EditObj.TargetX,
              y = this.game.EditObj.TargetY
            }))
              flag2 = true;
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 11)
        {
          str = "Preparing Land Artillery Attack. Select participants. ";
          if ( this.game.Data.RuleVar[834] > 0.0)
            str = str + Conversion.Str( (this.game.HandyFunctionsObj.CurrentAttackStackart() + this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].get_BattleStackArt(this.game.Data.Turn))) + "/" + Conversion.Str( this.game.Data.RuleVar[834]);
          if (this.game.EditObj.UnitSelected > -1)
          {
            if (this.game.HandyFunctionsObj.CanDoArtAttack(this.game.EditObj.UnitSelected, Coordinate::new()
            {
              x = this.game.EditObj.TargetX,
              y = this.game.EditObj.TargetY
            }, false))
              flag2 = true;
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 13)
        {
          str = "Preparing Shore Bombardment. Select participants. " + Conversion.Str( (this.game.HandyFunctionsObj.CurrentAttackStackart() + this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].get_BattleStackArt(this.game.Data.Turn))) + "/" + Conversion.Str( this.game.Data.RuleVar[834]);
          if (this.game.EditObj.UnitSelected > -1)
          {
            if (this.game.HandyFunctionsObj.CanDoSeaArtAttack(this.game.EditObj.UnitSelected, Coordinate::new()
            {
              x = this.game.EditObj.TargetX,
              y = this.game.EditObj.TargetY
            }))
              flag2 = true;
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 12)
        {
          str = "Preparing Sea Attack. Select participants.";
          if (this.game.EditObj.UnitSelected > -1)
          {
            if (this.game.HandyFunctionsObj.CanDoSeaAttack(this.game.EditObj.UnitSelected, Coordinate::new()
            {
              x = this.game.EditObj.TargetX,
              y = this.game.EditObj.TargetY
            }))
              flag2 = true;
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 4)
        {
          str = "Select a HQ for " + this.game.Data.LocObj[this.game.EditObj.OrderLoc].Name;
          if (this.game.Data.LocObj[this.game.EditObj.OrderLoc].HQ > -1)
          {
            this.KillId = 1;
            tDescript = "Set location to have no hq.";
          }
          if (this.game.EditObj.UnitSelected > -1 && this.game.HandyFunctionsObj.CanUnitBecomeHQforLoc(this.game.EditObj.UnitSelected, this.game.EditObj.OrderLoc))
            flag2 = true;
        }
        if (this.game.EditObj.OrderType == 20)
        {
          str = "Select cargo for " + this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Name;
          if (this.game.EditObj.UnitSelected > -1 && this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit)
          {
            this.NotOkText = "Unit already has 8 subformations.";
            if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].SFCount + this.game.Data.UnitObj[this.game.EditObj.OrderUnit].PassengerCounter + 1 < 7)
            {
              this.NotOkText = "Unit is empty.";
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount > -1)
              {
                this.NotOkText = "Unit is not yours";
                if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn | this.game.Data.Round == 0)
                {
                  let mut num: i32 =  0;
                  if (this.game.HandyFunctionsObj.IsHexPort(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap))
                  {
                    if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) > 0)
                    {
                      num = 1;
                      this.NotOkText = "Unit is not in port.";
                    }
                  }
                  else if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) > 1)
                  {
                    num = 1;
                    this.NotOkText = "Unit is not next to naval unit.";
                  }
                  if (num == 0)
                  {
                    this.NotOkText = "Unit may not have naval or aerial subformations.";
                    if (!this.game.HandyFunctionsObj.HasUnitNavySF(this.game.EditObj.UnitSelected) & !this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected))
                    {
                      this.NotOkText = "Unit is to heavy.";
                      if (this.game.HandyFunctionsObj.GetUnitWeight(this.game.EditObj.UnitSelected, true) <= this.game.HandyFunctionsObj.GetUnitCarryCap(this.game.EditObj.OrderUnit, 1, true))
                        flag2 = true;
                    }
                  }
                }
              }
            }
          }
        }
        if (this.game.EditObj.OrderType == 21)
        {
          str = "Select disembark hex for " + this.game.Data.UnitObj[this.game.EditObj.OrderTarget].Name;
          if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea)
          {
            if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) == 0)
            {
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter < 15)
                flag2 = true;
            }
            else if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) <= 1)
            {
              this.NotOkText = "Landscape not suitable";
              if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].CanAmph && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.OrderMap].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].LandscapeType].IsSea)
              {
                if (this.game.HandyFunctionsObj.IsHostileNotSelf(this.game.Data.Turn, this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime))
                {
                  this.NotOkText = "Unit needs at least " + Conversion.Str( this.game.Data.RuleVar[307]) + " power points to do amphibious invasion.";
                  if ( this.game.Data.RuleVar[307] <=  this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.game.EditObj.OrderTarget))
                    flag2 = true;
                }
                else if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime, this.game.Data.Turn) && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter < 15)
                  flag2 = true;
              }
            }
          }
        }
        if (this.AllId > 0 & this.lastorderx == this.game.EditObj.OrderX & this.lastordery == this.game.EditObj.OrderY)
          this.AllId = 0;
        SizeF sizeF2 = Expression.MeasureString(str, this.game.VicFont1);
        let mut num35: i32 =  this.OwnBitmap.Width;
        SubPartClass tsubpart;
        if (flag1)
        {
          tsubpart =  new SteveButtonPartClass(this.game.BACKBUTTON, tDescript: "Back to Main Menu [ESC]", tBackbitmap: ( this.OwnBitmap), bbx: 1, bby: 4);
          this.Cancelid = this.AddSubPart( tsubpart, 1, 4, 32, 32, 1);
        }
        if (this.game.EditObj.OrderType == 2 | this.game.EditObj.OrderType == 11 | this.game.EditObj.OrderType == 14 | this.game.EditObj.OrderType == 13 | this.game.EditObj.OrderType == 12 | this.game.EditObj.OrderType == 15)
        {
          let mut num36: i32 =  num35 - 40;
          if (this.AllId == 1)
          {
            tsubpart =  new SteveButtonPartClass(this.game.ALLBUTTON, tDescript: "Select all eligble units to join attack", tBackbitmap: ( this.OwnBitmap), bbx: num36, bby: 4);
            this.AllId = this.AddSubPart( tsubpart, num36, 4, 32, 32, 1);
          }
          else
          {
            tsubpart =  new SteveButtonPartClass(this.game.ALLBUTTON, 1, "Option not available",  this.OwnBitmap, num36, 4);
            this.All2Id = this.AddSubPart( tsubpart, num36, 4, 32, 32, 0);
          }
          let mut num37: i32 =  num36 - 40;
          if (this.NoneId == 1)
          {
            tsubpart =  new SteveButtonPartClass(this.game.NONEBUTTON, tDescript: "Remove all selected units from attack", tBackbitmap: ( this.OwnBitmap), bbx: num37, bby: 4);
            this.NoneId = this.AddSubPart( tsubpart, num37, 4, 32, 32, 1);
          }
          else
          {
            tsubpart =  new SteveButtonPartClass(this.game.NONEBUTTON, 1, "Option not available",  this.OwnBitmap, num37, 4);
            this.None2Id = this.AddSubPart( tsubpart, num37, 4, 32, 32, 0);
          }
          num35 = num37 - 104;
          if (flag3)
          {
            tsubpart =  new TextButtonPartClass("ATTACK", 96, "Start Battle!!",  this.OwnBitmap, num35, 4);
            this.BattleId = this.AddSubPart( tsubpart, num35 + 4, 4, 96, 32, 1);
          }
          else
          {
            tsubpart =  new TextButtonPartClass("ATTACK", 96, "You cannot start a battle",  this.OwnBitmap, num35, 4, true);
            this.Battle2Id = this.AddSubPart( tsubpart, num35 + 4, 4, 96, 32, 1);
          }
        }
        if (this.PopupButtonId > 0)
        {
          num35 -= 50;
          tsubpart =  new TextButtonPartClass("LIST", 48, "Select from list",  this.OwnBitmap, num35, 4);
          this.PopupButtonId = this.AddSubPart( tsubpart, num35, 4, 48, 32, 1);
        }
        if (this.KillId == 1)
        {
          num35 -= 50;
          tsubpart =  new SteveButtonPartClass(this.game.NONEBUTTON, tDescript: tDescript, tBackbitmap: ( this.OwnBitmap), bbx: num35, bby: 4);
          this.KillId = this.AddSubPart( tsubpart, num35, 4, 32, 32, 1);
        }
        let mut num38: i32 =  num35 - 40;
        if (flag2)
        {
          tsubpart =  new SteveButtonPartClass(this.game.OKBALL, tDescript: "Select / Deselect [SPACE]", tBackbitmap: ( this.OwnBitmap), bbx: num38, bby: 4);
          this.OkId = this.AddSubPart( tsubpart, num38, 4, 32, 32, 1);
        }
        else
        {
          tsubpart =  new SteveButtonPartClass(this.game.OKBALL, 1, this.NotOkText,  this.OwnBitmap, num38, 4);
          this.Ok2Id = this.AddSubPart( tsubpart, num38, 4, 32, 32, 0);
        }
        let mut x: i32 =   Math.Round( this.OwnBitmap.Width / 2.0 -  sizeF2.Width / 2.0);
        if ( x +  sizeF2.Width >  (num38 - 20))
          x =  Math.Round( ( x - ( x + sizeF2.Width -  (num38 - 20))));
        DrawMod.DrawTextVic( Expression, str, this.game.VicFont1, x, 12, this.game.VicColor1, this.game.VicColor1Shade);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (this.game.EditObj.BattleTimerActive)
        return windowReturnClass1;
      try
      {
        windowReturnClass2: WindowReturnClass;
        if (this.game.EditObj.OrderType == 1 && nr == 71 & this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical > -1)
        {
          if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical].StartSize > 1)
          {
            this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
            windowReturnClass2 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
            windowReturnClass3: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.GroupMoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.GroupMoveButtonId)] + 1, 1);
            this.game.EditObj.TempCoordList = CoordList::new();
            windowReturnClass3.SetFlag(true);
            return windowReturnClass3;
          }
          this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
          windowReturnClass2 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
          windowReturnClass4: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.MoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.MoveButtonId)] + 1, 1);
          this.game.EditObj.TempCoordList = CoordList::new();
          windowReturnClass4.SetFlag(true);
          return windowReturnClass4;
        }
        if (this.game.EditObj.OrderType == 48 && nr == 77)
        {
          this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
          windowReturnClass2 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
          windowReturnClass5: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.MoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.MoveButtonId)] + 1, 1);
          this.game.EditObj.TempCoordList = CoordList::new();
          windowReturnClass5.SetFlag(true);
          return windowReturnClass5;
        }
        if (this.game.EditObj.OrderType == 0)
        {
          if (nr == 77 & this.MoveButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.MoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.MoveButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 71)
          {
            if (this.GroupMoveButtonId > 0)
            {
              windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.GroupMoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.GroupMoveButtonId)] + 1, 1);
              windowReturnClass1.SetFlag(true);
            }
            else if (this.MoveButtonId > 0)
            {
              windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.MoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.MoveButtonId)] + 1, 1);
              windowReturnClass1.SetFlag(true);
            }
          }
          if (nr == 84 & this.TransferButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.TransferButtonId)] + 1, this.SubPartY[this.SubpartNr(this.TransferButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 83 & this.StrategicButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.StrategicButtonId)] + 1, this.SubPartY[this.SubpartNr(this.StrategicButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 72 & this.HqUnitButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.HqUnitButtonId)] + 1, this.SubPartY[this.SubpartNr(this.HqUnitButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 78 & this.NewUnitButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.NewUnitButtonId)] + 1, this.SubPartY[this.SubpartNr(this.NewUnitButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 79 & this.HqProdButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.HqProdButtonId)] + 1, this.SubPartY[this.SubpartNr(this.HqProdButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 80 & this.ProdButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ProdButtonId)] + 1, this.SubPartY[this.SubpartNr(this.ProdButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 112 & this.DipId > 0)
          {
            windowReturnClass6: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.DipId)] + 1, this.SubPartY[this.SubpartNr(this.DipId)] + 1, 1);
            windowReturnClass6.SetFlag(true);
            return windowReturnClass6;
          }
          if (nr == 113 & this.ResearchId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ResearchId)] + 1, this.SubPartY[this.SubpartNr(this.ResearchId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 117 & this.HexUnitButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.HexUnitButtonId)] + 1, this.SubPartY[this.SubpartNr(this.HexUnitButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 118 & this.ButtonStackedUnitId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ButtonStackedUnitId)] + 1, this.SubPartY[this.SubpartNr(this.ButtonStackedUnitId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 114 & this.StatisticsButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.StatisticsButtonId)] + 1, this.SubPartY[this.SubpartNr(this.StatisticsButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 115 & this.HistoryId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.HistoryId)] + 1, this.SubPartY[this.SubpartNr(this.HistoryId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 116 & this.SupplyLayerButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.SupplyLayerButtonId)] + 1, this.SubPartY[this.SubpartNr(this.SupplyLayerButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if ((nr == 191 | nr == 187 | nr == 107) & this.ButtonZoomInId > 0 & this.game.EditObj.Zoom < 1)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ButtonZoomInId)] + 1, this.SubPartY[this.SubpartNr(this.ButtonZoomInId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if ((nr == 219 | nr == 189 | nr == 109) & this.ButtonZoomOutId > 0 & this.game.EditObj.Zoom > -1)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ButtonZoomOutId)] + 1, this.SubPartY[this.SubpartNr(this.ButtonZoomOutId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (this.game.SelectX > -1)
          {
            if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea)
            {
              if (nr == 65 & this.AttackButtonId > 0)
              {
                windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.AttackButtonId)] + 1, this.SubPartY[this.SubpartNr(this.AttackButtonId)] + 1, 1);
                windowReturnClass1.SetFlag(true);
              }
              if (nr == 66 & this.ArtAttackButtonId > 0)
              {
                windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ArtAttackButtonId)] + 1, this.SubPartY[this.SubpartNr(this.ArtAttackButtonId)] + 1, 1);
                windowReturnClass1.SetFlag(true);
              }
            }
            else
            {
              if (nr == 65 & this.seaAttackButtonId > 0)
              {
                windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.seaAttackButtonId)] + 1, this.SubPartY[this.SubpartNr(this.seaAttackButtonId)] + 1, 1);
                windowReturnClass1.SetFlag(true);
              }
              if (nr == 66 & this.ArtAttackButtonId > 0)
              {
                windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ArtAttackButtonId)] + 1, this.SubPartY[this.SubpartNr(this.ArtAttackButtonId)] + 1, 1);
                windowReturnClass1.SetFlag(true);
              }
            }
            if (nr == 90 & this.AirAttackButtonId > 0)
            {
              windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.AirAttackButtonId)] + 1, this.SubPartY[this.SubpartNr(this.AirAttackButtonId)] + 1, 1);
              windowReturnClass1.SetFlag(true);
            }
            if (nr == 88 & this.InterdictButtonId > 0)
            {
              windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.InterdictButtonId)] + 1, this.SubPartY[this.SubpartNr(this.InterdictButtonId)] + 1, 1);
              windowReturnClass1.SetFlag(true);
            }
          }
          if (nr == 76 & this.LoadButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.LoadButtonId)] + 1, this.SubPartY[this.SubpartNr(this.LoadButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 85 & this.UnLoadButtonID > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.UnLoadButtonID)] + 1, this.SubPartY[this.SubpartNr(this.UnLoadButtonID)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 82 & this.InfraButtonId > 0)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.InfraButtonId)] + 1, this.SubPartY[this.SubpartNr(this.InfraButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 27 & this.SupplyLayerButtonId > 0 & this.game.EditObj.LayerSupplyOn)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.SupplyLayerButtonId)] + 1, this.SubPartY[this.SubpartNr(this.SupplyLayerButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
        }
        if (this.game.EditObj.OrderType > 0 & this.Cancelid > 0 && nr == 27)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (this.game.EditObj.OrderType > 0 & this.OkId == 0 & this.BattleId > 0)
        {
          if (this.game.SelectX == this.game.EditObj.TargetX & this.game.SelectY == this.game.EditObj.TargetY && nr == 32)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.BattleId)] + 1, this.SubPartY[this.SubpartNr(this.BattleId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          if (nr == 65)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.BattleId)] + 1, this.SubPartY[this.SubpartNr(this.BattleId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
        }
        if (!windowReturnClass1.Flag & this.game.EditObj.OrderType == 1 && nr == 32)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (this.game.EditObj.OrderType == 48 && nr == 32)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (this.game.EditObj.OrderType > 0 & this.OkId > 0)
        {
          if (nr == 32)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.OkId)] + 1, this.SubPartY[this.SubpartNr(this.OkId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        windowReturnClass1 = WindowReturnClass::new();
        windowReturnClass1.AddCommand(4, 44);
        windowReturnClass1.SetFlag(true);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass1;
    }

    pub fn PopUpRefresh() => this.DoRefresh();

    pub fn HighLightAItest()
    {
      let mut unitSelected: i32 =  this.game.EditObj.UnitSelected;
      this.game.EditObj.TempCoordList = CoordList::new();
      let mut num: i32 =  -1;
      let mut moveMatrixCounter: i32 =  this.game.NewAIObj.MoveMatrixCounter;
      for (let mut index: i32 =  0; index <= moveMatrixCounter; index += 1)
      {
        if (this.game.NewAIObj.MoveMatrixUnit[index] == unitSelected)
        {
          num = index;
          break;
        }
      }
      this.game.HandyFunctionsObj.RedimTempValue(9999);
      if (num <= -1)
        return;
      if (this.game.Data.UnitObj[unitSelected].TempCategory == 1)
      {
        let mut counter: i32 =  this.game.NewAIObj.MarkerList.Counter;
        for (let mut index: i32 =  0; index <= counter; index += 1)
          this.game.EditObj.TempValue[0].Value[this.game.NewAIObj.MarkerList.Data1[index], this.game.NewAIObj.MarkerList.Data2[index]] = 0;
      }
      else if (this.game.Data.UnitObj[unitSelected].TempCategory == 2)
      {
        let mut counter: i32 =  this.game.NewAIObj.ArtMarkerList.Counter;
        for (let mut index: i32 =  0; index <= counter; index += 1)
          this.game.EditObj.TempValue[0].Value[this.game.NewAIObj.ArtMarkerList.Data1[index], this.game.NewAIObj.ArtMarkerList.Data2[index]] = 0;
      }
      else if (this.game.Data.UnitObj[unitSelected].TempCategory == 3)
      {
        let mut counter: i32 =  this.game.NewAIObj.AirMarkerList.Counter;
        for (let mut index: i32 =  0; index <= counter; index += 1)
          this.game.EditObj.TempValue[0].Value[this.game.NewAIObj.AirMarkerList.Data1[index], this.game.NewAIObj.AirMarkerList.Data2[index]] = 0;
      }
      else
      {
        if (this.game.Data.UnitObj[unitSelected].TempCategory != 4)
          return;
        if (this.game.NewAIObj.EngineerMarkerList.Counter > -1)
        {
          let mut counter: i32 =  this.game.NewAIObj.EngineerMarkerList.Counter;
          for (let mut index: i32 =  0; index <= counter; index += 1)
            this.game.EditObj.TempValue[0].Value[this.game.NewAIObj.EngineerMarkerList.Data1[index], this.game.NewAIObj.EngineerMarkerList.Data2[index]] = 0;
        }
        else
        {
          let mut counter: i32 =  this.game.NewAIObj.MarkerList.Counter;
          for (let mut index: i32 =  0; index <= counter; index += 1)
            this.game.EditObj.TempValue[0].Value[this.game.NewAIObj.MarkerList.Data1[index], this.game.NewAIObj.MarkerList.Data2[index]] = 0;
        }
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      numArray: Vec<i32> = new int[this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1, this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1];
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.game.EditObj.BattleTimerActive)
        return windowReturnClass;
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            OrderResult orderResult1;
            if (num1 == this.BattleId)
            {
              this.game.EditObj.BattleTimerActive = false;
              this.game.EditObj.BattleAnimNr = 0;
              this.game.TempCombat = new CombatClass(this.game);
              orderResult1 = this.game.TempCombat.Init(Coordinate::new()
              {
                x = this.game.EditObj.TargetX,
                y = this.game.EditObj.TargetY,
                map = this.game.EditObj.TargetMap
              }, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType);
              windowReturnClass.AddCommand(3, 5);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.KillId)
            {
              switch (this.game.EditObj.OrderType)
              {
                case 3:
                  if (this.game.ProcessingObj.SetUnitHq(this.game.EditObj.OrderUnit, -1).OK)
                  {
                    this.game.EditObj.OrderType = 0;
                    this.game.EditObj.TempCoordList = CoordList::new();
                    this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                    windowReturnClass.AddCommand(4, 12);
                    windowReturnClass.AddCommand(4, 20);
                    windowReturnClass.AddCommand(4, 18);
                    this.game.SelectX = this.game.EditObj.OrderX;
                    this.game.SelectY = this.game.EditObj.OrderY;
                    this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
                case 4:
                  if (this.game.ProcessingObj.SetProdHq(this.game.EditObj.OrderLoc, -1).OK)
                  {
                    this.game.EditObj.OrderType = 0;
                    this.game.ProcessingObj.LocationProductionPrognosis();
                    windowReturnClass.AddCommand(4, 66);
                    windowReturnClass.AddCommand(4, 44);
                    windowReturnClass.AddCommand(4, 18);
                    this.game.EditObj.TempCoordList = CoordList::new();
                    this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    this.game.SelectX = this.game.EditObj.OrderX;
                    this.game.SelectY = this.game.EditObj.OrderY;
                    this.game.EditObj.MapSelected = this.game.EditObj.OrderMap;
                    this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                    windowReturnClass.AddCommand(4, 12);
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
              }
            }
            else if (num1 == this.NoneId)
            {
              this.game.EditObj.TempCoordList = CoordList::new();
              this.lastorderx = -1;
              this.lastordery = -1;
              let mut counter: i32 =  this.game.EditObj.TempUnitList.counter;
              for (let mut index2: i32 =  0; index2 <= counter; index2 += 1)
                this.game.EditObj.TempCoordList.AddCoord(this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index2]].X, this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index2]].Y, this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index2]].Map);
              this.game.EditObj.TempUnitList = UnitList::new();
              windowReturnClass.AddCommand(4, 18);
              windowReturnClass.AddCommand(4, 20);
              windowReturnClass.AddCommand(4, 12);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.AllId)
            {
              Coordinate target;
              target.x = this.game.EditObj.OrderX;
              target.y = this.game.EditObj.OrderY;
              this.lastorderx = target.x;
              this.lastordery = target.y;
              target.onmap = true;
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.EditObj.TempUnitList = UnitList::new();
              if (this.game.EditObj.OrderType == 11 | this.game.EditObj.OrderType == 13 | this.game.EditObj.OrderType == 14 | this.game.EditObj.OrderType == 15)
              {
                let mut mapCounter: i32 =  this.game.Data.MapCounter;
                for (let mut index3: i32 =  0; index3 <= mapCounter; index3 += 1)
                {
                  let mut mapWidth: i32 =  this.game.Data.MapObj[index3].MapWidth;
                  for (let mut index4: i32 =  0; index4 <= mapWidth; index4 += 1)
                  {
                    let mut mapHeight: i32 =  this.game.Data.MapObj[index3].MapHeight;
                    for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
                    {
                      Coordinate coordinate;
                      coordinate.x = index4;
                      coordinate.y = index5;
                      coordinate.map = index3;
                      coordinate.onmap = true;
                      if (this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter > -1 && this.game.Data.UnitObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitList[0]].Regime == this.game.Data.Turn)
                      {
                        let mut unitCounter: i32 =  this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter;
                        for (let mut index6: i32 =  0; index6 <= unitCounter; index6 += 1)
                        {
                          let mut unit: i32 =  this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitList[index6];
                          if (this.game.EditObj.OrderType == 11)
                          {
                            if (this.game.HandyFunctionsObj.CanDoArtAttack(unit, target, false))
                            {
                              this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                              this.game.EditObj.TempUnitList.add(unit);
                            }
                          }
                          else if (this.game.EditObj.OrderType == 13)
                          {
                            if (this.game.HandyFunctionsObj.CanDoSeaArtAttack(unit, target))
                            {
                              this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                              this.game.EditObj.TempUnitList.add(unit);
                            }
                          }
                          else if (this.game.HandyFunctionsObj.CanDoAirStrike(unit, target))
                          {
                            this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                            this.game.EditObj.TempUnitList.add(unit);
                          }
                        }
                      }
                    }
                  }
                }
              }
              else
              {
                let mut num2: i32 =  this.game.HandyFunctionsObj.HexNeighbourCount(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                for (let mut tfacing: i32 =  1; tfacing <= num2; tfacing += 1)
                {
                  Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, tfacing);
                  if (coordinate.onmap && this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter > -1 && this.game.Data.UnitObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitList[0]].Regime == this.game.Data.Turn)
                  {
                    let mut unitCounter: i32 =  this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter;
                    for (let mut index7: i32 =  0; index7 <= unitCounter; index7 += 1)
                    {
                      let mut unit: i32 =  this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitList[index7];
                      if (this.game.EditObj.OrderType == 2)
                      {
                        if (this.game.HandyFunctionsObj.CanDoLandAttack(unit, target))
                        {
                          this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                          this.game.EditObj.TempUnitList.add(unit);
                        }
                      }
                      else if (this.game.HandyFunctionsObj.CanDoSeaAttack(unit, target))
                      {
                        this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                        this.game.EditObj.TempUnitList.add(unit);
                      }
                    }
                  }
                }
              }
              windowReturnClass.AddCommand(4, 18);
              windowReturnClass.AddCommand(4, 20);
              windowReturnClass.AddCommand(4, 12);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.NextButtonId)
            {
              if (MsgBoxResult.Yes == Interaction.MsgBox( "End turn?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest"))
              {
                this.game.EditObj.UnitSelected = -1;
                this.game.EditObj.OrderUnit = -1;
                this.game.EditObj.OrderTarget = -1;
                if (this.game.HandyFunctionsObj.GetHumanPlayers() < 1)
                {
                  let mut num3: i32 =   Interaction.MsgBox( "Since you have surrendered we are quitting the game now.");
                  this.game.Data = DataClass::new();
                  this.game.EditObj = new EditClass(this.game.AppPath + "editobj.txt");
                  if (this.game.Data.UseAI == 1)
                    this.game.NewAIObj.LastRegime = -1;
                  this.game.EditObj.ShowInitialMenu = true;
                  windowReturnClass.AddCommand(3, 1);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                this.game.FormRef.Cursor = Cursors.WaitCursor;
                if (this.game.EditObj.Screenshoton)
                  this.game.HandyFunctionsObj.doscreenshot(nameof (b), 0);
                if (this.game.EditObj.AutoSave & !this.game.Data.PBEM)
                {
                  str: String = this.game.AppPath_SAVEGAMES + "autosave.se1";
                  this.game.Data.serialize(str);
                  this.game.HandyFunctionsObj.ZipFile(str);
                }
                windowReturnClass.AddCommand(3, 4);
                windowReturnClass.SetFlag(true);
              }
            }
            else if (num1 == this.OkId)
            {
              switch (this.game.EditObj.OrderType)
              {
                case 2:
                case 11:
                case 12:
                case 13:
                case 14:
                case 15:
                  if (this.game.EditObj.TempUnitList.CheckIfPresent(this.game.EditObj.UnitSelected))
                    this.game.EditObj.TempUnitList.remove(this.game.EditObj.UnitSelected);
                  else
                    this.game.EditObj.TempUnitList.add(this.game.EditObj.UnitSelected);
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  windowReturnClass.AddCommand(4, 18);
                  windowReturnClass.AddCommand(4, 20);
                  windowReturnClass.AddCommand(4, 12);
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
                case 3:
                  let mut historical: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical;
                  if (historical > -1 && this.game.Data.HistoricalUnitObj[historical].HandCardCounter > -1 & this.game.Data.UnitObj[this.game.EditObj.OrderUnit].SFCount > -1 && Interaction.MsgBox( "This action will cause the HQ to lose all handcards. Are you sure?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.No)
                  {
                    this.game.EditObj.OrderType = 0;
                    this.game.EditObj.TempCoordList = CoordList::new();
                    this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                    windowReturnClass.AddCommand(4, 12);
                    windowReturnClass.AddCommand(4, 20);
                    windowReturnClass.AddCommand(4, 18);
                    this.game.SelectX = this.game.EditObj.OrderX;
                    this.game.SelectY = this.game.EditObj.OrderY;
                    this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  OrderResult orderResult2 = this.game.ProcessingObj.SetUnitHq(this.game.EditObj.OrderUnit, this.game.EditObj.UnitSelected);
                  if (this.game.EditObj.SoundOn)
                    SoundMod.PlayAWave(this.game.AppPath + "sound/radio.wav",  this.game.EditObj);
                  if (orderResult2.OK)
                  {
                    this.game.EditObj.OrderType = 0;
                    this.game.EditObj.TempCoordList = CoordList::new();
                    windowReturnClass.AddCommand(4, 12);
                    windowReturnClass.AddCommand(4, 20);
                    windowReturnClass.AddCommand(4, 18);
                    this.game.SelectX = this.game.EditObj.OrderX;
                    this.game.SelectY = this.game.EditObj.OrderY;
                    this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
                case 4:
                  if (this.game.ProcessingObj.SetProdHq(this.game.EditObj.OrderLoc, this.game.EditObj.UnitSelected).OK)
                  {
                    this.game.EditObj.OrderType = 0;
                    this.game.ProcessingObj.LocationProductionPrognosis();
                    windowReturnClass.AddCommand(4, 66);
                    windowReturnClass.AddCommand(4, 44);
                    windowReturnClass.AddCommand(4, 18);
                    this.game.EditObj.TempCoordList = CoordList::new();
                    this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    this.game.SelectX = this.game.EditObj.OrderX;
                    this.game.SelectY = this.game.EditObj.OrderY;
                    this.game.EditObj.MapSelected = this.game.EditObj.OrderMap;
                    this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                    windowReturnClass.AddCommand(4, 12);
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
                case 18:
                case 49:
                  if (this.game.EditObj.OrderTarget == -1)
                  {
                    this.game.EditObj.OrderTarget = this.game.EditObj.UnitSelected;
                    this.game.EditObj.TempCoordList = CoordList::new();
                    this.game.EditObj.OrderX = this.game.SelectX;
                    this.game.EditObj.OrderY = this.game.SelectY;
                    windowReturnClass.AddCommand(1, 5);
                    windowReturnClass.AddCommand(4, 18);
                    windowReturnClass.AddCommand(2, 35);
                    windowReturnClass.AddCommand(4, 12);
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
                case 19:
                case 42:
                  if (this.game.EditObj.OrderTarget == -1)
                  {
                    this.game.EditObj.OrderTarget = this.game.EditObj.UnitSelected;
                    if (this.game.HandyFunctionsObj.HasUnitOnlyParaLandSF(this.game.EditObj.OrderTarget))
                    {
                      if (Interaction.MsgBox( "Do you want to paradrop?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                      {
                        this.game.EditObj.OrderType = 19;
                        this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, false, PredictAirOnly: true, ClearSea: true, attack: true, isparadrop: true);
                      }
                      else
                      {
                        this.game.EditObj.OrderType = 42;
                        this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, ClearSea: true, ismove: true);
                      }
                    }
                    else
                    {
                      this.game.EditObj.OrderType = 42;
                      this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, ClearSea: true, ismove: true);
                    }
                    windowReturnClass.AddCommand(4, 12);
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    break;
                  }
                  let mut num4: i32 =  0;
                  if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn)
                  {
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > 14)
                    {
                      num4 = 1;
                      let mut num5: i32 =   Interaction.MsgBox( "Already 16 units in that hex.");
                      this.game.EditObj.OrderType = 0;
                      windowReturnClass.AddCommand(4, 12);
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 20);
                      this.game.SelectX = this.game.EditObj.OrderX;
                      this.game.SelectY = this.game.EditObj.OrderY;
                      this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                    }
                    this.game.HandyFunctionsObj.IsHexAirfield(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  }
                  if (num4 == 0)
                  {
                    this.game.EditObj.TargetX = this.game.SelectX;
                    this.game.EditObj.TargetY = this.game.SelectY;
                    this.game.TempCombat = new CombatClass(this.game);
                    Coordinate Target = Coordinate::new();
                    Target.x = this.game.EditObj.TargetX;
                    Target.y = this.game.EditObj.TargetY;
                    this.game.SelectX = this.game.EditObj.TargetX;
                    this.game.SelectY = this.game.EditObj.TargetY;
                    this.game.EditObj.TempUnitList = UnitList::new();
                    this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderUnit);
                    if (this.game.EditObj.OrderType != 42)
                      this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderTarget, 1);
                    if (this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
                      windowReturnClass.AddCommand(3, 5);
                    windowReturnClass.SetFlag(true);
                    break;
                  }
                  break;
                case 20:
                  OrderResult orderResult3 = this.game.ProcessingObj.LoadUnit(this.game.EditObj.UnitSelected, this.game.EditObj.OrderUnit);
                  if (this.game.EditObj.SoundOn)
                    SoundMod.PlayAWave(this.game.AppPath + "sound/load.wav",  this.game.EditObj);
                  if (orderResult3.OK)
                  {
                    this.game.EditObj.OrderType = 0;
                    this.game.EditObj.TempCoordList = CoordList::new();
                    windowReturnClass.AddCommand(4, 12);
                    windowReturnClass.AddCommand(4, 44);
                    windowReturnClass.AddCommand(4, 20);
                    windowReturnClass.AddCommand(4, 18);
                    this.game.SelectX = this.game.EditObj.OrderX;
                    this.game.SelectY = this.game.EditObj.OrderY;
                    if (this.game.EditObj.MapSelected != this.game.EditObj.OrderMap)
                    {
                      this.game.EditObj.MapSelected = this.game.EditObj.OrderMap;
                      this.game.EditObj.TempCoordList = CoordList::new();
                    }
                    else
                      this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    this.game.EditObj.TempCoordList = CoordList::new();
                    this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
                case 21:
                  OrderResult orderResult4 = this.game.ProcessingObj.unLoadUnit(this.game.EditObj.OrderTarget, this.game.EditObj.OrderUnit, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  if (this.game.EditObj.SoundOn)
                    SoundMod.PlayAWave(this.game.AppPath + "sound/unload.wav",  this.game.EditObj);
                  if (orderResult4.OK)
                  {
                    if (!this.game.HandyFunctionsObj.VisibleEnemyUnitsInHex(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.Data.Turn, true) & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].CardUponConquest == -1)
                    {
                      this.game.EditObj.OrderType = 0;
                      windowReturnClass.AddCommand(4, 12);
                      windowReturnClass.AddCommand(4, 44);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 18);
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.game.EditObj.UnitSelected = this.game.EditObj.OrderTarget;
                      this.game.EditObj.OrderType = 0;
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                      break;
                    }
                    this.game.EditObj.TargetX = this.game.SelectX;
                    this.game.EditObj.TargetY = this.game.SelectY;
                    this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                    this.game.TempCombat = new CombatClass(this.game);
                    Coordinate Target = Coordinate::new();
                    Target.x = this.game.EditObj.TargetX;
                    Target.y = this.game.EditObj.TargetY;
                    Target.map = this.game.EditObj.TargetMap;
                    this.game.EditObj.TempUnitList = UnitList::new();
                    this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderTarget);
                    if (this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
                      windowReturnClass.AddCommand(3, 5);
                    windowReturnClass.SetFlag(true);
                    break;
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
                case 33:
                  this.game.TempCombat = new CombatClass(this.game);
                  Coordinate Target1 = Coordinate::new();
                  Target1.x = this.game.SelectX;
                  Target1.y = this.game.SelectY;
                  this.game.EditObj.TempUnitList = UnitList::new();
                  this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderUnit);
                  if (this.game.TempCombat.Init(Target1, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
                    windowReturnClass.AddCommand(3, 5);
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
                case 34:
                  this.game.EditObj.OrderLoc = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
                  this.game.EditObj.TargetX = this.game.SelectX;
                  this.game.EditObj.TargetY = this.game.SelectY;
                  this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(1, 5);
                  windowReturnClass.AddCommand(1, 2);
                  windowReturnClass.AddCommand(2, 47);
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
                case 35:
                  if (this.game.ProcessingObj.BlowBridge(this.game.EditObj.OrderUnit, this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) - 1).OK)
                  {
                    SoundMod.PlayAWave(this.game.AppPath + "sound/blow.wav",  this.game.EditObj);
                    this.game.EditObj.OrderType = 0;
                    windowReturnClass.AddCommand(4, 12);
                    windowReturnClass.AddCommand(4, 44);
                    windowReturnClass.AddCommand(4, 20);
                    windowReturnClass.AddCommand(4, 29);
                    windowReturnClass.AddCommand(4, 66);
                    this.game.SelectX = this.game.EditObj.OrderX;
                    this.game.SelectY = this.game.EditObj.OrderY;
                    this.game.EditObj.MapSelected = this.game.EditObj.OrderMap;
                    this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.OrderMap);
                    this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                  }
                  else
                  {
                    this.game.EditObj.OrderType = 0;
                    windowReturnClass.AddCommand(4, 12);
                    windowReturnClass.AddCommand(4, 44);
                    windowReturnClass.AddCommand(4, 20);
                    this.game.SelectX = this.game.EditObj.OrderX;
                    this.game.SelectY = this.game.EditObj.OrderY;
                    this.game.EditObj.MapSelected = this.game.EditObj.OrderMap;
                    this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                    windowReturnClass.AddCommand(4, 29);
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
                case 36:
                  OrderResult orderResult5 = this.game.ProcessingObj.BuildInfra(this.game.EditObj.OrderUnit, this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) - 1);
                  if (this.game.EditObj.SoundOn)
                    SoundMod.PlayAWave(this.game.AppPath + "sound/building.wav",  this.game.EditObj);
                  if (orderResult5.OK)
                  {
                    this.game.EditObj.OrderType = 0;
                    this.game.EditObj.TempCoordList = CoordList::new();
                    this.game.ProcessingObj.LocationProductionPrognosis();
                    windowReturnClass.AddCommand(4, 66);
                    windowReturnClass.AddCommand(4, 12);
                    windowReturnClass.AddCommand(4, 44);
                    windowReturnClass.AddCommand(4, 20);
                    this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
                case 40:
                  this.game.EditObj.TargetX = this.game.SelectX;
                  this.game.EditObj.TargetY = this.game.SelectY;
                  this.game.EditObj.TempCoordList = CoordList::new();
                  windowReturnClass.AddCommand(1, 5);
                  windowReturnClass.AddCommand(2, 51);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.SetFlag(true);
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  break;
              }
            }
            else if (num1 == this.HexUnitButtonId)
            {
              if (this.game.EditObj.HideUnit == 0)
                this.game.EditObj.HideUnit = 1;
              else if (this.game.EditObj.HideUnit == 1)
              {
                if ( this.game.Data.RuleVar[344] == 1.0)
                  this.game.EditObj.HideUnit = 2;
                else
                  this.game.EditObj.HideUnit = 0;
              }
              else
                this.game.EditObj.HideUnit = 0;
              this.dostuff();
              this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 20);
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.ButtonZoomOutId)
            {
              let mut num6: i32 =   Math.Round(Conversion.Int( (this.game.ScreenWidth - 200) / 53.0));
              let mut num7: i32 =   Math.Round(Conversion.Int( (this.game.ScreenWidth - 200) / 106.0));
              let mut num8: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - 265) / 53.0));
              let mut num9: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - 265) / 106.0));
              num10: i32;
              num11: i32;
              if (this.game.EditObj.Zoom == 0)
              {
                this.game.EditObj.Zoom = -1;
                this.game.CornerX -=  Math.Round(Conversion.Int( num6 / 2.0));
                this.game.CornerY -=  Math.Round(Conversion.Int( num8 / 2.0));
                num10 = 27;
                num11 = 24;
              }
              else
              {
                this.game.EditObj.Zoom = 0;
                this.game.CornerX -=  Math.Round(Conversion.Int( num7 / 2.0));
                this.game.CornerY -=  Math.Round(Conversion.Int( num9 / 2.0));
                num10 = 53;
                num11 = 48;
              }
              if ( this.game.CornerX +  (this.game.ScreenWidth - 200) /  num10 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
                this.game.CornerX =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth) -  (this.game.ScreenWidth - 200) /  num10);
              if ( this.game.CornerY +  (this.game.ScreenHeight - 256) /  num11 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
                this.game.CornerY =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) -  (this.game.ScreenHeight - 256) /  num11);
              if (this.game.CornerX < 0)
                this.game.CornerX = 0;
              if (this.game.CornerY < 0)
                this.game.CornerY = 0;
              this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
              this.dostuff();
              this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(1, 3);
              windowReturnClass.AddCommand(2, 12);
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.ButtonZoomInId)
            {
              let mut num12: i32 =   Math.Round(Conversion.Int( (this.game.ScreenWidth - 200) / 53.0));
              let mut num13: i32 =   Math.Round(Conversion.Int( (this.game.ScreenWidth - 200) / 106.0));
              let mut num14: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - 265) / 53.0));
              let mut num15: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - 265) / 106.0));
              num16: i32;
              num17: i32;
              if (this.game.EditObj.Zoom == 0)
              {
                this.game.EditObj.Zoom = 1;
                this.game.CornerX +=  Math.Round(Conversion.Int( num13 / 2.0));
                this.game.CornerY +=  Math.Round(Conversion.Int( num15 / 2.0));
                num16 = 106;
                num17 = 96;
              }
              else
              {
                this.game.EditObj.Zoom = 0;
                this.game.CornerX +=  Math.Round(Conversion.Int( num12 / 2.0));
                this.game.CornerY +=  Math.Round(Conversion.Int( num14 / 2.0));
                num16 = 53;
                num17 = 48;
              }
              if ( this.game.CornerX +  (this.game.ScreenWidth - 200) /  num16 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
                this.game.CornerX =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth) -  (this.game.ScreenWidth - 200) /  num16);
              if ( this.game.CornerY +  (this.game.ScreenHeight - 256) /  num17 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
                this.game.CornerY =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) -  (this.game.ScreenHeight - 256) /  num17);
              if (this.game.CornerX < 0)
                this.game.CornerX = 0;
              if (this.game.CornerY < 0)
                this.game.CornerY = 0;
              this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
              this.dostuff();
              this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(1, 3);
              windowReturnClass.AddCommand(2, 12);
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.ButtonStackedUnitId)
            {
              this.game.EditObj.SpreadUnit = !this.game.EditObj.SpreadUnit;
              this.dostuff();
              this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 20);
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.SupplyLayerButtonId)
            {
              if (this.game.EditObj.LayerSupplyOn)
              {
                this.game.EditObj.LayerSupplyOn = false;
              }
              else
              {
                this.game.EditObj.LayerSupplyOn = true;
                let mut unr: i32 =  this.game.EditObj.UnitSelected;
                if (unr != -1)
                {
                  if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
                  {
                    if (!this.game.Data.UnitObj[unr].IsHQ)
                      unr = this.game.Data.UnitObj[unr].HQ;
                    this.game.EditObj.LayerSupplyHQ = unr;
                  }
                  else
                    this.game.EditObj.LayerSupplyHQ = -1;
                }
                else
                  this.game.EditObj.LayerSupplyHQ = -1;
                if (this.game.EditObj.LayerSupplyHQ == -1)
                  this.game.HandyFunctionsObj.MakeSupplyLayer2(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                else
                  this.game.HandyFunctionsObj.MakeSupplyLayer(unr);
              }
              this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(4, 12);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.GroupMoveButtonId)
            {
              this.game.EditObj.OrderType = 48;
              this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePredictionGroup(this.game.EditObj.UnitSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, attackoptions: true, ismove: true);
              this.game.EditObj.TempCoordList.RemoveCoord(0);
              this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
              Coordinate coordinate = this.game.HandyFunctionsObj.SetTempCanSee(this.game.EditObj.TempCoordList);
              if (coordinate.onmap)
              {
                this.game.EditObj.MapSelected = coordinate.map;
                this.game.SelectX = coordinate.x;
                this.game.SelectY = coordinate.y;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.UnitSelected = this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, true, 0);
                windowReturnClass.AddCommand(4, 20);
              }
              this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
              windowReturnClass.AddCommand(4, 18);
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 29);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.MoveButtonId)
            {
              this.game.EditObj.OrderType = 1;
              this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.UnitSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, attackoptions: true, ismove: true);
              this.game.EditObj.TempCoordList.RemoveCoord(0);
              this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
              Coordinate coordinate = this.game.HandyFunctionsObj.SetTempCanSee(this.game.EditObj.TempCoordList);
              if (coordinate.onmap & this.game.EditObj.MapSelected != coordinate.map)
              {
                this.game.EditObj.MapSelected = coordinate.map;
                this.game.SelectX = coordinate.x;
                this.game.SelectY = coordinate.y;
                this.game.EditObj.MouseOverX = coordinate.x;
                this.game.EditObj.MouseOverX = coordinate.y;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.UnitSelected = this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, true, 0);
                windowReturnClass.AddCommand(4, 20);
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 29);
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
              windowReturnClass.AddCommand(4, 18);
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 29);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.UnLoadButtonID)
            {
              this.game.EditObj.OrderTarget = -1;
              this.game.EditObj.OrderType = 21;
              this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
              this.game.EditObj.OrderTarget = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerList[this.game.EditObj.SFSelected - (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount + 1)];
              this.game.EditObj.OrderX = this.game.SelectX;
              this.game.EditObj.OrderY = this.game.SelectY;
              this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.EditObj.TempValue = new MapMatrix2[this.game.Data.MapCounter + 1];
              let mut mapCounter1: i32 =  this.game.Data.MapCounter;
              for (let mut index8: i32 =  0; index8 <= mapCounter1; index8 += 1)
                this.game.EditObj.TempValue[index8] = new MapMatrix2(this.game.Data.MapObj[index8].MapWidth, this.game.Data.MapObj[index8].MapHeight);
              let mut mapCounter2: i32 =  this.game.Data.MapCounter;
              for (let mut index9: i32 =  0; index9 <= mapCounter2; index9 += 1)
              {
                let mut mapWidth: i32 =  this.game.Data.MapObj[index9].MapWidth;
                for (let mut index10: i32 =  0; index10 <= mapWidth; index10 += 1)
                {
                  let mut mapHeight: i32 =  this.game.Data.MapObj[index9].MapHeight;
                  for (let mut index11: i32 =  0; index11 <= mapHeight; index11 += 1)
                    this.game.EditObj.TempValue[index9].Value[index10, index11] = 9999;
                }
              }
              let mut num18: i32 =  this.game.HandyFunctionsObj.HexNeighbourCount(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
              for (let mut tfacing: i32 =  1; tfacing <= num18; tfacing += 1)
              {
                Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, tfacing);
                if (coordinate.onmap && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                {
                  if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, coordinate.x, coordinate.y, coordinate.map) == 0)
                  {
                    if (this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter < 15)
                    {
                      this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                      this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                    }
                  }
                  else if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, coordinate.x, coordinate.y, coordinate.map) <= 1 && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].LandscapeType].CanAmph && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].LandscapeType].IsSea)
                  {
                    if (this.game.HandyFunctionsObj.IsHostileNotSelf(this.game.Data.Turn, this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].Regime))
                    {
                      this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                      this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                    }
                    else if (this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].Regime == this.game.Data.Turn && this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter < 15)
                    {
                      this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                      this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                    }
                  }
                }
              }
              Coordinate coordinate1 = this.game.HandyFunctionsObj.SetTempCanSee(this.game.EditObj.TempCoordList);
              if (coordinate1.onmap)
              {
                this.game.EditObj.MapSelected = coordinate1.map;
                this.game.SelectX = coordinate1.x;
                this.game.SelectY = coordinate1.y;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.UnitSelected = this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, true, 0);
                windowReturnClass.AddCommand(4, 18);
              }
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 29);
              windowReturnClass.AddCommand(4, 20);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.LoadButtonId)
            {
              this.game.EditObj.OrderTarget = -1;
              this.game.EditObj.OrderType = 20;
              this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
              this.game.EditObj.OrderX = this.game.SelectX;
              this.game.EditObj.OrderY = this.game.SelectY;
              this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.EditObj.TempValue = new MapMatrix2[this.game.Data.MapCounter + 1];
              let mut mapCounter3: i32 =  this.game.Data.MapCounter;
              for (let mut index12: i32 =  0; index12 <= mapCounter3; index12 += 1)
                this.game.EditObj.TempValue[index12] = new MapMatrix2(this.game.Data.MapObj[index12].MapWidth, this.game.Data.MapObj[index12].MapHeight);
              let mut mapCounter4: i32 =  this.game.Data.MapCounter;
              for (let mut index13: i32 =  0; index13 <= mapCounter4; index13 += 1)
              {
                let mut mapWidth: i32 =  this.game.Data.MapObj[index13].MapWidth;
                for (let mut index14: i32 =  0; index14 <= mapWidth; index14 += 1)
                {
                  let mut mapHeight: i32 =  this.game.Data.MapObj[index13].MapHeight;
                  for (let mut index15: i32 =  0; index15 <= mapHeight; index15 += 1)
                    this.game.EditObj.TempValue[index13].Value[index14, index15] = 9999;
                }
              }
              let mut num19: i32 =  this.game.HandyFunctionsObj.HexNeighbourCount(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
              for (let mut tfacing: i32 =  1; tfacing <= num19; tfacing += 1)
              {
                Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, tfacing);
                if (coordinate.onmap && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                {
                  if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, coordinate.x, coordinate.y, coordinate.map) == 0)
                  {
                    if (this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter < 15)
                    {
                      this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                      this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                    }
                  }
                  else if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, coordinate.x, coordinate.y, coordinate.map) <= 1 && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].LandscapeType].CanAmph && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].LandscapeType].IsSea)
                  {
                    if (this.game.HandyFunctionsObj.IsHostileNotSelf(this.game.Data.Turn, this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].Regime))
                    {
                      this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                      this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                    }
                    else if (this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].Regime == this.game.Data.Turn && this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter < 15)
                    {
                      this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                      this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                    }
                  }
                }
              }
              Coordinate coordinate2 = this.game.HandyFunctionsObj.SetTempCanSee(this.game.EditObj.TempCoordList);
              if (coordinate2.onmap)
              {
                this.game.EditObj.MapSelected = coordinate2.map;
                this.game.SelectX = coordinate2.x;
                this.game.SelectY = coordinate2.y;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.UnitSelected = this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, true, 0);
                windowReturnClass.AddCommand(4, 18);
              }
              this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 29);
              windowReturnClass.AddCommand(4, 20);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.TransferButtonId)
            {
              this.game.EditObj.OrderTarget = -1;
              this.game.EditObj.OrderType = 9;
              this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
              this.game.EditObj.OrderX = this.game.SelectX;
              this.game.EditObj.OrderY = this.game.SelectY;
              this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
              this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(1, 5);
              windowReturnClass.AddCommand(4, 18);
              windowReturnClass.AddCommand(2, 30);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.StrategicButtonId)
            {
              this.game.EditObj.OrderTarget = -1;
              this.game.EditObj.TargetX = -1;
              this.game.EditObj.TargetY = -1;
              this.game.EditObj.OrderType = 18;
              this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
              this.game.EditObj.OrderX = this.game.SelectX;
              this.game.EditObj.OrderY = this.game.SelectY;
              this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
              let mut singleCapHq: i32 =  this.game.HandyFunctionsObj.GetSingleCapHQ();
              if (singleCapHq > -1)
              {
                this.game.EditObj.OrderTarget = singleCapHq;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(2, 35);
                windowReturnClass.AddCommand(4, 12);
              }
              else
                windowReturnClass.AddCommand(4, 18);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.GroupStrategicButtonId)
            {
              this.game.EditObj.OrderTarget = -1;
              this.game.EditObj.TargetX = -1;
              this.game.EditObj.TargetY = -1;
              this.game.EditObj.OrderType = 49;
              this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
              this.game.EditObj.OrderX = this.game.SelectX;
              this.game.EditObj.OrderY = this.game.SelectY;
              this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
              let mut singleCapHq: i32 =  this.game.HandyFunctionsObj.GetSingleCapHQ();
              if (singleCapHq > -1)
              {
                this.game.EditObj.OrderTarget = singleCapHq;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(2, 35);
                windowReturnClass.AddCommand(4, 12);
              }
              else
                windowReturnClass.AddCommand(4, 18);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.PrefsButtonId)
            {
              this.game.EditObj.OrderType = 30;
              this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
              this.game.EditObj.OrderX = this.game.SelectX;
              this.game.EditObj.OrderY = this.game.SelectY;
              windowReturnClass.AddCommand(1, 5);
              windowReturnClass.AddCommand(2, 45);
              this.dostuff();
              windowReturnClass.SetFlag(true);
            }
            else
            {
              if (num1 == this.StatisticsButtonId)
              {
                this.game.EditObj.OrderType = 0;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.PopupValue = 15;
                windowReturnClass.AddCommand(5, 10);
                this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.HqUnitButtonId)
              {
                this.game.EditObj.OrderType = 3;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BlowBridgeButtonId)
              {
                this.game.EditObj.OrderType = 35;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                this.game.HandyFunctionsObj.BlowBridgeHexHighlight(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.InfraButtonId)
              {
                this.game.EditObj.OrderType = 36;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.HandyFunctionsObj.InfraHexHighlight(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.EditObj.UnitSelected);
                windowReturnClass.AddCommand(4, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BuildButtonId)
              {
                bool flag;
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location == -1)
                {
                  this.game.EditObj.OrderType = 37;
                  this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                  this.game.EditObj.OrderX = this.game.SelectX;
                  this.game.EditObj.OrderY = this.game.SelectY;
                  this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                  windowReturnClass.AddCommand(1, 5);
                  windowReturnClass.AddCommand(2, 48);
                  this.dostuff();
                  flag = true;
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut location: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
                  if (this.game.Data.LocObj[location].StructuralPts >= this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].StructuralPts)
                  {
                    this.game.EditObj.OrderType = 37;
                    this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                    this.game.EditObj.OrderX = this.game.SelectX;
                    this.game.EditObj.OrderY = this.game.SelectY;
                    this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                    windowReturnClass.AddCommand(1, 5);
                    windowReturnClass.AddCommand(2, 48);
                    this.dostuff();
                    flag = true;
                    windowReturnClass.SetFlag(true);
                  }
                }
                if ( this.game.Data.RuleVar[902] < 1.0 & !flag & this.game.EditObj.UnitSelected > -1 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1)
                {
                  if (this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].StructuralPts < this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].StructuralPts)
                  {
                    orderResult1 = this.game.ProcessingObj.RepairLocation(this.game.EditObj.UnitSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    if (this.game.EditObj.SoundOn)
                      SoundMod.PlayAWave(this.game.AppPath + "sound/building.wav",  this.game.EditObj);
                    windowReturnClass.AddCommand(4, 20);
                    windowReturnClass.AddCommand(4, 18);
                    windowReturnClass.AddCommand(4, 66);
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                }
              }
              else if (num1 == this.NewUnitButtonId)
              {
                this.game.EditObj.OrderType = 7;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 28);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.OfficerId)
              {
                this.game.EditObj.OrderType = 45;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 61);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.NewUnitButton2Id)
              {
                this.game.EditObj.OrderType = 44;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 59);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.MakeHQButtonID)
              {
                this.game.EditObj.OrderType = 0;
                this.game.ProcessingObj.DoMakeHQ(this.game.EditObj.UnitSelected);
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 20);
                windowReturnClass.AddCommand(4, 18);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.ParadropButtonId)
              {
                this.game.EditObj.OrderTarget = -1;
                this.game.EditObj.TargetX = -1;
                this.game.EditObj.TargetY = -1;
                this.game.EditObj.OrderType = 19;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 20);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.AirReconButtonId)
              {
                this.game.EditObj.OrderTarget = -1;
                this.game.EditObj.TargetX = -1;
                this.game.EditObj.TargetY = -1;
                this.game.EditObj.OrderType = 33;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, false, PredictAirOnly: true, attack: true, OnlyFrontline: true);
                this.game.EditObj.TempValue3 = (MapMatrix2[]) this.game.EditObj.TempValue.Clone();
                this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, false, PredictAirOnly: true, attack: true);
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 20);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.AirSupplyButtonId)
              {
                this.game.EditObj.OrderTarget = -1;
                this.game.EditObj.TargetX = -1;
                this.game.EditObj.TargetY = -1;
                this.game.EditObj.OrderType = 40;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, false, PredictAirOnly: true, attack: true);
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 20);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.HqProdButtonId)
              {
                this.game.EditObj.OrderType = 4;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderLoc = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.ProdButtonId)
              {
                this.game.EditObj.OrderType = 6;
                this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                this.game.EditObj.OrderLoc = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 25);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.AirAttackButtonId)
              {
                this.game.EditObj.OrderTarget = -1;
                this.game.EditObj.TempUnitList = UnitList::new();
                this.game.EditObj.OrderType = 14;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TargetX = this.game.SelectX;
                this.game.EditObj.TargetY = this.game.SelectY;
                this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 29);
                windowReturnClass.AddCommand(4, 20);
                windowReturnClass.AddCommand(4, 18);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.AttackButtonId)
              {
                this.game.EditObj.OrderTarget = -1;
                this.game.EditObj.TempUnitList = UnitList::new();
                this.game.EditObj.OrderType = 2;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TargetX = this.game.SelectX;
                this.game.EditObj.TargetY = this.game.SelectY;
                this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 29);
                windowReturnClass.AddCommand(4, 20);
                windowReturnClass.AddCommand(4, 18);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.seaAttackButtonId)
              {
                this.game.EditObj.OrderTarget = -1;
                this.game.EditObj.TempUnitList = UnitList::new();
                this.game.EditObj.OrderType = 12;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TargetX = this.game.SelectX;
                this.game.EditObj.TargetY = this.game.SelectY;
                this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 29);
                windowReturnClass.AddCommand(4, 20);
                windowReturnClass.AddCommand(4, 18);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.SeaArtAttackButtonId)
              {
                this.game.EditObj.OrderTarget = -1;
                this.game.EditObj.TempUnitList = UnitList::new();
                this.game.EditObj.OrderType = 13;
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TargetX = this.game.SelectX;
                this.game.EditObj.TargetY = this.game.SelectY;
                this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 29);
                windowReturnClass.AddCommand(4, 20);
                windowReturnClass.AddCommand(4, 18);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.ArtAttackButtonId)
              {
                this.game.EditObj.OrderTarget = -1;
                this.game.EditObj.OrderType = 11;
                this.game.EditObj.TempUnitList = UnitList::new();
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TargetX = this.game.SelectX;
                this.game.EditObj.TargetY = this.game.SelectY;
                this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 29);
                windowReturnClass.AddCommand(4, 20);
                windowReturnClass.AddCommand(4, 18);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.InterdictButtonId)
              {
                this.game.EditObj.OrderTarget = -1;
                this.game.EditObj.OrderType = 15;
                this.game.EditObj.OrderBombMode = 1;
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 & this.game.HandyFunctionsObj.HasHexBridge(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected))
                {
                  if (!this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].Invincible)
                  {
                    if (Interaction.MsgBox( "Attack the location/town? (answer 'No' if you want to attack the bridge(s))", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                      this.game.EditObj.OrderBombMode = 1;
                    else
                      this.game.EditObj.OrderBombMode = 0;
                  }
                }
                else
                  this.game.EditObj.OrderBombMode = 0;
                this.game.EditObj.TempUnitList = UnitList::new();
                this.game.EditObj.OrderX = this.game.SelectX;
                this.game.EditObj.OrderY = this.game.SelectY;
                this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TargetX = this.game.SelectX;
                this.game.EditObj.TargetY = this.game.SelectY;
                this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 29);
                windowReturnClass.AddCommand(4, 20);
                windowReturnClass.AddCommand(4, 18);
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else
              {
                if (num1 == this.disbandid)
                {
                  this.game.ProcessingObj.DoDisbandUnit(this.game.EditObj.UnitSelected);
                  if (this.game.EditObj.SoundOn)
                    SoundMod.PlayAWave(this.game.AppPath + "sound/disband.wav",  this.game.EditObj);
                  this.game.EditObj.UnitSelected = -1;
                  if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > -1)
                    this.game.EditObj.UnitSelected = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitList[0];
                  this.dostuff();
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 20);
                  windowReturnClass.AddCommand(4, 18);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.GiveUnitId)
                {
                  Form3::new( this.formref).Initialize(this.game.Data, 52, this.game.EditObj.UnitSelected);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.GiveHexId)
                {
                  tnr: i32;
                  if (Interaction.MsgBox( "Set a radius of hexes to give? Say no if you only want to give this hex.", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                  {
                    tnr =  Math.Round(Conversion.Val(Interaction.InputBox("Give a radius of hexes to give. 1-99", "Shadow Empire : Planetary Conquest")));
                    if (tnr < 1 | tnr > 99)
                    {
                      let mut num20: i32 =   Interaction.MsgBox( "wrong input. aborting giving of hex(es).", Title: ( "Shadow Empire : Planetary Conquest"));
                      return windowReturnClass;
                    }
                  }
                  else
                    tnr = 0;
                  Form3::new( this.formref).Initialize(this.game.Data, 53, tnr);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.SFDesignButtonId)
                {
                  this.game.EditObj.OrderType = 52;
                  windowReturnClass.AddCommand(1, 5);
                  windowReturnClass.AddCommand(2, 65);
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  if (num1 == this.ResearchId)
                  {
                    this.game.EditObj.OrderType = 23;
                    this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                    this.game.EditObj.PopupValue = 14;
                    windowReturnClass.AddCommand(5, 10);
                    this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 == this.ModelDesignerId)
                  {
                    this.game.EditObj.OrderType = 47;
                    windowReturnClass.AddCommand(3, 8);
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                  }
                  else if (num1 == this.ChangeModelId)
                  {
                    this.game.EditObj.OrderType = 46;
                    this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                    this.game.EditObj.OrderX = this.game.SelectX;
                    this.game.EditObj.OrderY = this.game.SelectY;
                    this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                    windowReturnClass.AddCommand(1, 5);
                    windowReturnClass.AddCommand(2, 63);
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                  }
                  else if (num1 == this.DipId)
                  {
                    this.game.EditObj.OrderType = 24;
                    this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                    windowReturnClass.AddCommand(1, 5);
                    windowReturnClass.AddCommand(1, 3);
                    windowReturnClass.AddCommand(1, 66);
                    windowReturnClass.AddCommand(2, 39);
                    windowReturnClass.AddCommand(2, 53);
                    windowReturnClass.AddCommand(2, 66);
                    windowReturnClass.AddCommand(1, 2);
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                  }
                  else if (num1 == this.HistoryId)
                  {
                    this.game.EditObj.LayerSupplyOn = false;
                    this.game.EditObj.OrderType = 26;
                    windowReturnClass.AddCommand(3, 6);
                    windowReturnClass.SetFlag(true);
                  }
                  else if (num1 == this.BlowLocationButtonId)
                  {
                    if (Interaction.MsgBox( "Are you Sure?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                    {
                      this.game.ProcessingObj.BlowLocation(this.game.EditObj.UnitSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      SoundMod.PlayAWave(this.game.AppPath + "sound/blow.wav",  this.game.EditObj);
                      windowReturnClass.AddCommand(4, 12);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 66);
                      this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                    }
                  }
                  else if (num1 == this.OrderSurrenderButtonId)
                  {
                    if (Interaction.MsgBox( "Are you Sure you wish to surrender?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                    {
                      this.game.EditObj.UnitSelected = -1;
                      this.game.EditObj.OrderUnit = -1;
                      this.game.EditObj.OrderTarget = -1;
                      let mut humanPlayers: i32 =  this.game.HandyFunctionsObj.GetHumanPlayers();
                      if (humanPlayers == 0)
                      {
                        if (Interaction.MsgBox( "Are you sure? Next turn will mean you quit this surrendered game...", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                        {
                          this.game.Data = DataClass::new();
                          this.game.EditObj = new EditClass(this.game.AppPath + "editobj.txt");
                          if (this.game.Data.UseAI == 1)
                            this.game.NewAIObj.LastRegime = -1;
                          this.game.EditObj.ShowInitialMenu = true;
                          windowReturnClass.AddCommand(3, 1);
                          windowReturnClass.SetFlag(true);
                        }
                      }
                      else
                      {
                        this.game.EditObj.TempCoordList = CoordList::new();
                        windowReturnClass.AddCommand(4, 12);
                      }
                      switch (humanPlayers)
                      {
                        case 1:
                          this.game.Data.Winner = this.game.HandyFunctionsObj.FindAIPlayer();
                          this.game.Data.RegimeObj[this.game.Data.Turn].Sleep = true;
                          break;
                        case 2:
                          this.game.Data.Winner = this.game.HandyFunctionsObj.FindOtherHumanPlayer(this.game.Data.Turn);
                          this.game.Data.RegimeObj[this.game.Data.Turn].Sleep = true;
                          break;
                        default:
                          this.game.EventRelatedObj.ExecJoinRegime(this.game.Data.Turn, -1, 0, 0, "");
                          this.game.Data.RegimeObj[this.game.Data.Turn].Sleep = true;
                          break;
                      }
                      for (let mut unitCounter: i32 =  this.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
                      {
                        if (this.game.Data.UnitObj[unitCounter].Regime == this.game.Data.Turn & this.game.Data.UnitObj[unitCounter].PreDef == -1)
                        {
                          data: DataClass = this.game.Data;
                          let mut nr: i32 =  unitCounter;
                          let mut gameClass: GameClass = (GameClass) null;
                           let mut local: GameClass =  gameClass;
                          data.RemoveUnit(nr,  local);
                        }
                      }
                      this.game.EventRelatedObj.ExecMessage2(-1, -1, -1, -1, this.game.Data.RegimeObj[this.game.Data.Turn].Name + " has surrendered.");
                      if (humanPlayers > 2)
                      {
                        let mut num21: i32 =   Interaction.MsgBox( ("You have surrendered. There are " + Conversion.Str( (humanPlayers - 1)) + " other human players left. Dont forget to save and mail the turn if you play PBEM."), Title: ( "Shadow Empire : Planetary Conquest"));
                        windowReturnClass.AddCommand(3, 4);
                        windowReturnClass.SetFlag(true);
                      }
                      else
                      {
                        let mut num22: i32 =   Interaction.MsgBox( "You have surrendered. All statistics are available now.", Title: ( "Shadow Empire : Planetary Conquest"));
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                    }
                  }
                  else if (num1 == this.PopupButtonId)
                  {
                    switch (this.game.EditObj.OrderType)
                    {
                      case 3:
                        this.TimerUsed = false;
                        Form3::new( this.formref).Initialize(this.game.Data, 82, this.game.EditObj.OrderUnit, tGame: this.game);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      case 14:
                      case 15:
                        this.TimerUsed = false;
                        Form3::new( this.formref).Initialize(this.game.Data, 59, this.game.EditObj.UnitSelected, tGame: this.game);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      case 18:
                      case 49:
                        this.TimerUsed = false;
                        Form3::new( this.formref).Initialize(this.game.Data, 60, this.game.EditObj.OrderUnit, tGame: this.game);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                    }
                  }
                  else if (num1 == this.Cancelid)
                  {
                    switch (this.game.EditObj.OrderType)
                    {
                      case 1:
                      case 43:
                      case 48:
                        this.game.EditObj.OrderType = 0;
                        if (this.game.EditObj.TempCoordList.counter < 3)
                          this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        this.game.SelectX = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X;
                        this.game.SelectY = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y;
                        this.game.EditObj.MapSelected = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map;
                        windowReturnClass.AddCommand(4, 12);
                        windowReturnClass.AddCommand(4, 29);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 2:
                      case 11:
                      case 12:
                      case 13:
                      case 14:
                      case 15:
                        this.game.EditObj.OrderType = 0;
                        this.lastorderx = -1;
                        this.lastordery = -1;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.TargetX, this.game.EditObj.TargetY, this.game.EditObj.MapSelected);
                        if (this.game.EditObj.TempUnitList.counter > -1)
                        {
                          let mut counter: i32 =  this.game.EditObj.TempUnitList.counter;
                          for (let mut index16: i32 =  0; index16 <= counter; index16 += 1)
                            this.game.EditObj.TempCoordList.AddCoord(this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index16]].X, this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index16]].Y, this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index16]].Map);
                        }
                        this.game.EditObj.TargetX = -1;
                        this.game.EditObj.TargetY = -1;
                        windowReturnClass.AddCommand(4, 12);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 3:
                      case 4:
                      case 5:
                        this.game.EditObj.OrderType = 0;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                        windowReturnClass.AddCommand(4, 12);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        if (this.game.Data.Round == 0)
                          this.game.Data.Turn = -1;
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 6:
                        this.game.EditObj.OrderType = 0;
                        if (this.game.EditObj.ProdFlap)
                        {
                          this.game.EditObj.ProdFlap = false;
                          windowReturnClass.AddCommand(1, 3);
                          windowReturnClass.AddCommand(2, 12);
                          windowReturnClass.AddCommand(2, 18);
                          windowReturnClass.AddCommand(1, 66);
                          windowReturnClass.AddCommand(2, 66);
                          windowReturnClass.AddCommand(4, 29);
                        }
                        else
                          windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(1, 5);
                        windowReturnClass.AddCommand(2, 20);
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 7:
                      case 44:
                        this.game.EditObj.OrderType = 0;
                        windowReturnClass.AddCommand(1, 5);
                        windowReturnClass.AddCommand(2, 20);
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 8:
                        this.game.EditObj.OrderType = 0;
                        windowReturnClass.AddCommand(1, 5);
                        windowReturnClass.AddCommand(2, 18);
                        windowReturnClass.AddCommand(2, 20);
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 9:
                        this.game.EditObj.OrderType = 0;
                        this.game.EditObj.ShowTransfer = false;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                        this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.TargetX, this.game.EditObj.TargetY, this.game.EditObj.TargetMap);
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        windowReturnClass.AddCommand(4, 12);
                        windowReturnClass.AddCommand(1, 5);
                        windowReturnClass.AddCommand(2, 20);
                        windowReturnClass.AddCommand(4, 18);
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 18:
                      case 49:
                        this.game.EditObj.ShowTransfer = false;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        windowReturnClass.AddCommand(4, 12);
                        windowReturnClass.AddCommand(1, 5);
                        windowReturnClass.AddCommand(2, 20);
                        windowReturnClass.AddCommand(4, 18);
                        this.game.EditObj.OrderType = 0;
                        this.game.EditObj.TargetX = -1;
                        this.game.EditObj.TargetY = -1;
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 19:
                      case 42:
                        this.game.EditObj.OrderType = 0;
                        windowReturnClass.AddCommand(4, 12);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 20);
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 20:
                      case 21:
                        this.game.EditObj.TempCoordList = CoordList::new();
                        let mut mapCounter: i32 =  this.game.Data.MapCounter;
                        for (let mut index17: i32 =  0; index17 <= mapCounter; index17 += 1)
                        {
                          let mut mapWidth: i32 =  this.game.Data.MapObj[index17].MapWidth;
                          for (let mut index18: i32 =  0; index18 <= mapWidth; index18 += 1)
                          {
                            let mut mapHeight: i32 =  this.game.Data.MapObj[index17].MapHeight;
                            for (let mut index19: i32 =  0; index19 <= mapHeight; index19 += 1)
                              this.game.EditObj.TempValue[index17].Value[index18, index19] = 9999;
                          }
                        }
                        this.game.EditObj.OrderType = 0;
                        windowReturnClass.AddCommand(4, 12);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 20);
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.MapSelected = this.game.EditObj.OrderMap;
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 22:
                      case 30:
                        this.game.EditObj.OrderType = 0;
                        windowReturnClass.AddCommand(1, 5);
                        windowReturnClass.AddCommand(2, 20);
                        if (this.game.Data.Round == 0)
                          this.game.Data.Turn = -1;
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 23:
                        this.game.EditObj.OrderType = 0;
                        windowReturnClass.AddCommand(1, 5);
                        windowReturnClass.AddCommand(2, 20);
                        windowReturnClass.AddCommand(2, 18);
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 24:
                        this.game.EditObj.OrderType = 0;
                        windowReturnClass.AddCommand(1, 5);
                        windowReturnClass.AddCommand(1, 3);
                        windowReturnClass.AddCommand(1, 66);
                        windowReturnClass.AddCommand(2, 66);
                        windowReturnClass.AddCommand(2, 12);
                        windowReturnClass.AddCommand(2, 20);
                        windowReturnClass.AddCommand(2, 18);
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 25:
                        this.game.EditObj.OrderType = 0;
                        windowReturnClass.AddCommand(1, 5);
                        windowReturnClass.AddCommand(2, 20);
                        windowReturnClass.AddCommand(2, 18);
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 33:
                      case 35:
                      case 36:
                      case 39:
                        this.game.EditObj.TempCoordList = CoordList::new();
                        windowReturnClass.AddCommand(4, 12);
                        windowReturnClass.AddCommand(4, 20);
                        this.game.EditObj.OrderType = 0;
                        this.game.EditObj.TargetX = -1;
                        this.game.EditObj.TargetY = -1;
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 34:
                        this.game.EditObj.OrderType = 0;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        if (this.game.EditObj.TargetX > -1)
                        {
                          this.game.EditObj.TempCoordList.AddCoord(this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map);
                          this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.TargetX, this.game.EditObj.TargetY, this.game.EditObj.TargetMap);
                          windowReturnClass.AddCommand(4, 12);
                          windowReturnClass.AddCommand(1, 5);
                          windowReturnClass.AddCommand(2, 20);
                          windowReturnClass.AddCommand(2, 18);
                        }
                        else
                        {
                          this.game.EditObj.TempCoordList.AddCoord(this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map);
                          windowReturnClass.AddCommand(4, 12);
                          windowReturnClass.AddCommand(4, 20);
                        }
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.TargetX = -1;
                        this.game.EditObj.TargetY = -1;
                        this.game.EditObj.OrderLoc = -1;
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 37:
                        this.game.EditObj.OrderType = 0;
                        if (this.game.EditObj.ProdFlap)
                        {
                          this.game.EditObj.ProdFlap = false;
                          windowReturnClass.AddCommand(1, 3);
                          windowReturnClass.AddCommand(2, 12);
                        }
                        windowReturnClass.AddCommand(1, 5);
                        windowReturnClass.AddCommand(2, 20);
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 40:
                        this.game.EditObj.OrderType = 0;
                        if (this.game.EditObj.TargetX > -1)
                        {
                          windowReturnClass.AddCommand(4, 12);
                          windowReturnClass.AddCommand(1, 5);
                          windowReturnClass.AddCommand(2, 20);
                        }
                        else
                        {
                          windowReturnClass.AddCommand(4, 12);
                          windowReturnClass.AddCommand(4, 20);
                        }
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.TargetX = -1;
                        this.game.EditObj.TargetY = -1;
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                      case 45:
                      case 46:
                      case 52:
                        this.game.EditObj.OrderType = 0;
                        windowReturnClass.AddCommand(1, 5);
                        windowReturnClass.AddCommand(2, 20);
                        if (this.game.Data.Round == 0)
                          this.game.Data.Turn = -1;
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        break;
                    }
                  }
                }
              }
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
